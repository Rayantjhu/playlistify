// remove when functions are complete
#![allow(dead_code)] 
#![allow(unused_variables)]

use pkce;

use error_chain::error_chain;
use std::io::Read;
use rand::{thread_rng, Rng};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn request_oath() {
    /*  
    Using PKCE to grant authorization. 

    STEP 1:
        Request authorization from the user
        this can be done using a GET request to the `/authorize` endpoint
    */
}



pub fn example_request() -> Result<()> {
    let mut response = reqwest::blocking::get("https://httpbin.org/get")?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;

    println!("Body:\n{}", body);

    Ok(())
}

pub fn req_user_auth() {
    let client_id = "1b2956db25984895bf349523fb4d559d"; // ID of the registered application
    let response_type = "code";

    // TODO: Come up with some way to maybe create a localhost site where the user is redirected to 
    // UPDATE: not necessary to do the above, could just redirect to localhost/
    let redirect_uri = ""; // URI redirected to when the user denies/grants permission (needs to have been entered in the allowlist)

    let state = ""; // Optional, but recommended. 

    // Used for PKCE extension
    let code_challenge_method = "S256"; 

    // The code must be hashed using SHA256. Random string between 43 and 128 characters. it may contain: 
    //      [a-z] [A-Z] [0-9] [_ . - ~]
    //      letters, digits, underscores, 
    //      periods, hyphens or tildes                                            
    let code_challenge = generate_challenge();
}

fn generate_challenge() -> String {
    pkce::code_challenge(&pkce::code_verifier(128))
}
