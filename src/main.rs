#![allow(dead_code)]
mod http;
mod server;
mod website_handler;

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let publc_path = env::var("PUBLIC_PATH").unwrap_or(default_path.to_string());
    println!("Public path is {}", &default_path);
    // Because we're using a mod, we must prepend the mod name to reference the method
    let server = Server::new("127.0.0.1:8080".to_string());

    server.run(WebsiteHandler::new(publc_path));
}
