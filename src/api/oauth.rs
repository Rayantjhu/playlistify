use log::{error, warn};
use rspotify::{prelude::*, scopes, AuthCodePkceSpotify, Credentials, OAuth};

#[tokio::main]
pub async fn get_spotify_client() -> Result<AuthCodePkceSpotify, String> {
    let creds = match Credentials::from_env() {
        Some(creds) => creds,
        None => {
            warn!("Unable to find client_id in the env variables.");
            return Err("Unable to find client_id in the env variables.".to_string());
        }
    };
    let oauth = match OAuth::from_env(scopes!("user-read-playback-state")) {
        Some(oauth) => oauth,
        None => {
            warn!("Unable to find redirect uri in the env variables.");
            return Err("Unable to find redirect uri in the env variables.".to_string());
        }
    };
    let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());

    let url = match spotify.get_authorize_url(Some(128)) {
        Ok(url) => url,
        Err(e) => {
            error!("Error occurred when retrieving authorization url: {e}");
            return Err(format!(
                "An error occurred while retrieving authorization url: {}".to_string(),
                e
            ));
        }
    };

    match spotify.prompt_for_token(&url).await {
        Ok(()) => (),
        Err(e) => {
            warn!("Something went wrong when prompting for the auth token: {e}");
            return Err(format!(
                "Something went wrong when prompting for the authentication token: {}".to_string(),
                e
            ));
        }
    };

    return Ok(spotify);
}
