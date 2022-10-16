#![allow(unused_imports)]

mod api;
mod conf;

use crate::api::oauth::example_request;
use crate::conf::set_env_vars;
use std::env;

fn main() {
    set_env_vars();

    api::oauth::request_oath();
}
