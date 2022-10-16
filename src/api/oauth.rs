// TODO: remove when functions are complete
#![allow(dead_code)]
#![allow(unused_variables)]

use pkce;
use std::env;

use error_chain::error_chain;
use futures::executor::block_on;
use open;
use reqwest::Url;
use std::io;
use std::io::Read;
use std::process::exit;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
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
pub fn request_oath() {
    let client_id = match env::var("CLIENT_ID") {
        Ok(v) => v,
        Err(e) => panic!("Environment variable {e} not defined."),
    };
    let redirect_uri = match env::var("REDIRECT_URI") {
        Ok(v) => v,
        Err(e) => panic!("Environment variable {e} not defined."),
    };

    let code_verifier = pkce::code_verifier(128);
    let code_challenge = pkce::code_challenge(&code_verifier);
    let code_verifier = std::str::from_utf8(&code_verifier).unwrap();

    // Step 1
    block_on(request_user_auth(
        &client_id,
        &code_challenge,
        &redirect_uri,
    ));
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
/// * `client_id`: [`&str`](std::str) - The Client ID from the application
/// * `code_challenge`: [`&str`](std::str) - The code challenge used for the PKCE method
/// * `redirect_uri`: [`&str`](std::str) - The URI which the user will be redirect to once the
///                                         authorization has been completed
///
/// See the [`Spotify Docs`] about authorization for more information
///
/// [`/authorize`]:https://accounts.spotify.com/authorize
/// [`Spotify Docs`]:https://developer.spotify.com/documentation/general/guides/authorization/code-flow/
async fn request_user_auth(client_id: &str, code_challenge: &str, redirect_uri: &str) {
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

    // The URL for the GET request
    let url = match Url::parse_with_params("https://accounts.spotify.com/authorize", &params) {
        Ok(v) => v,
        Err(e) => panic!("Cannot parse {e}"),
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
///
/// [`/api/token`]:https://accounts.spotify.com/api/token
fn request_access_token(client_id: &str, code_verifier: &str, redirect_uri: &str) {
    let grant_type = "authorization_code";
    let code = ""; // The previously returned code from step 1
    let redirect_uri = ""; // Used for validation, it must match the URI used in step 1

    /*
    The following HTTP headers must be included:
    -   Authorization: Base 64 encoded string that contains the client ID and secret, must follow the
        format:
        Authorization: Basic <base64 encoded client_id:client_secret>
    -   Content-Type: set to application/x-www-form-urlencoded
    */
    // TODO: Implement step 2
}

// TODO: Delete when example is not needed anymore
/// An example of a valid GET request
pub fn example_request() -> Result<()> {
    let mut response = reqwest::blocking::get("https://httpbin.org/get")?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;

    println!("Body:\n{}", body);

    Ok(())
}
