use pkce;

use error_chain::error_chain;
use futures::executor::block_on;
use open;
use reqwest::header::HeaderMap;
use reqwest::{ RequestBuilder};
use reqwest::Url;
use std::{env, error, io};
use std::env::VarError;
use std::fmt::Error;
use std::io::Read;
use std::process::exit;
use std::string::ParseError;
use serde::Deserialize;

#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u32,
    refresh_token: String,
}

/*
    Using PKCE to grant authorization.

    STEP 1:
        Request authorization from the user
        this can be done using a GET request to the `/authorize` endpoint
    STEP 2:
        Exchange the authorization code for an Access Token.
        This can ben done by making a POST request to the /api/token endpoint
*/
pub fn request_oath() -> Result<(), Box<dyn error::Error>> {
    let client_id = env::var("CLIENT_ID")?;
    let redirect_uri = env::var("REDIRECT_URI")?;

    let code_verifier = pkce::code_verifier(128);
    let code_challenge = pkce::code_challenge(&code_verifier);
    let code_verifier = std::str::from_utf8(&code_verifier).unwrap();

    // Step 1
    let code = match block_on(request_user_auth(
        &client_id,
        &code_challenge,
        &redirect_uri,
    )) {
        Ok(v) => v,
        Err(e) => panic!("Error occurred: {}", e),
    };


    // Step 2
    let access_token = match block_on(request_access_token(
        &client_id,
        &code_verifier,
        &redirect_uri,
        &code
    )) {
        Ok(v) => v,
        Err(e) => panic!("Error occurred while requesting the access token: {}", e)
    };

    Ok(())
}

/// # Requests the user's authorization.
///
/// The first step of using the Spotify API, is to request authorization from the user, so this
/// application may have access to the resources on behalf of the user.
///
/// <br/>
///
/// This is done by creating a GET request to the [`/authorize`] endpoint
///
/// <br/>
///
/// # Arguments
/// * `code_challenge`: [`&str`](std::str) - The code challenge used for the PKCE method
/// * `redirect_uri`: [`&str`](std::str) - The URI which the user will be redirect to once the
///                                         authorization has been completed
///
/// See the [`Spotify Docs`] about authorization for more information
///
/// [`/authorize`]:https://accounts.spotify.com/authorize
/// [`Spotify Docs`]:https://developer.spotify.com/documentation/general/guides/authorization/code-flow/
async fn request_user_auth(
    client_id: &str,
    code_challenge: &str,
    redirect_uri: &str,
) -> Result<String, Box<dyn error::Error>> {
    let state = ""; // Optional, but recommended.

    // The parameters used for the GET request
    let params = [
        ("client_id", client_id),
        ("response_type", "code"),
        ("redirect_uri", redirect_uri),
        ("state", state),
        ("code_challenge_method", "S256"),
        ("code_challenge", code_challenge),
    ];

    let auth_url = env::var("SPOTIFY_API_AUTHENTICATION")?;

    // The URL for the GET request
    let url = match Url::parse_with_params(&auth_url, &params) {
        Ok(v) => v,
        Err(e) => panic!("Cannot parse {}", e),
    };

    // Checking if the user wishes to authenticate or not
    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        println!("would you like to authorize your account. (y)es, (n)o?: ");

        // Clearing the string as it otherwise would append
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => break,
            "no" | "n" => {
                println!("User cancelled, exiting program.");
                exit(0);
            }
            _ => println!("Incorrect input, please try again"),
        };
    }

    println!("Opening the authentication page...");

    // Opening the browser with the GET request path, so the user may authenticate.
    match open::that(url.as_str()) {
        Ok(()) => {
            println!("Url successfully opened. Use the following link if it didn't open\n {url}")
        }
        Err(e) => println!("An error occurred while trying the open the {url}: {e}"),
    }

    let mut code = String::new();

    println!(
        "\n\
    If the authentication was successful, please enter the code that may be found in the URL.\n\
    e.g: localhost/code?{{your_code}}\n\
    If it wasn't successful, enter nothing: "
    );

    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read input");
    match code.as_str() {
        "" => {
            println!("Exiting program, user cancelled");
            exit(0);
        }
        _ => Ok(code),
    }
}

/// # Requests an access token
/// If the user has accepted the authorization request, the authorization code must be exchanged for
/// an <b>Access Token</b>.
///
/// <br/>
///
/// This is done by creating a POST request to the [`/api/token`] endpoint.
///
/// <br/>
///
/// # Arguments
/// * `client_id`: [`&str`](std::str) - The Client ID from the application
/// * `code_verifier`: [`&str`](std::str) - The code verifier used for the PKCE method
/// * `redirect_uri`: [`&str`](std::str) - The URI which the user will be redirect to once the
/// authorization has been completed. This must equal the redirect_uri used in the authorization
/// request
/// * `code`: [`&str`](std::str) - The code that was retrieved from the successful authorization
///
/// [`/api/token`]:https://accounts.spotify.com/api/token
async fn request_access_token(
    client_id: &str,
    code_verifier: &str,
    redirect_uri: &str,
    code: &str,
) -> Result<AccessTokenResponse, Box<dyn error::Error>> {
    /*
    The following HTTP headers must be included:
    -   Authorization: Base 64 encoded string that contains the client ID and secret, must follow the
        format:
        Authorization: Basic <base64 encoded client_id:client_secret>
    -   Content-Type: set to application/x-www-form-urlencoded
    "https://acounts.spotify.com/api/token
    */
    let params = [
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("code_verifier", code_verifier),
    ];

    let endpoint = env::var("SPOTIFY_API_ACCESS_TOKEN")?;
    let url = Url::parse_with_params(endpoint.as_str(), params)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    let response = match reqwest::blocking::Client::new()
        .post(url)
        .headers(headers)
        .send() {
        Ok(v) => v,
        Err(e) => panic!("Error occurred while requesting access token: {}", e)
    };

    if response.status().is_success() {
        let response_object: AccessTokenResponse = response.json().unwrap();
        return Ok(response_object)
    }

    // TODO: create my own error (response wasn't successful)
    Err(Box::new(Error))
}
