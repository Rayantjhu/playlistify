mod api;
mod conf;

use crate::conf::set_env_vars;
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok(); // Loads all the environment variables from the .env file

    api::oauth::request_oath();
}
