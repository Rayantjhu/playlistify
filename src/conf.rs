use std::env;

pub fn set_env_vars() {
    env::set_var("CLIENT_ID", "1b2956db25984895bf349523fb4d559d");
    env::set_var("REDIRECT_URI", "http://localhost/")
}