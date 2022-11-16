#![allow(dead_code)]
mod http;
mod server;
mod website_handler;

use http::method::Method;
use http::request::Request;
use server::Server;
use website_handler::WebsiteHandler;

fn main() {
    // Because we're using a mod, we must prepend the mod name to reference the method
    let server = Server::new("127.0.0.1:8080".to_string());

    server.run(WebsiteHandler);
}
