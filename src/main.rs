mod api;

use dotenv::dotenv;
use crate::api::oauth::get_spotify_client;

fn main() {
    dotenv().ok(); // Loads all the environment variables from the .env file

    match get_spotify_client() {
        Ok(spotify) => spotify,
        Err(e) => panic!("Something went wrong! {}", e)
    };
}
