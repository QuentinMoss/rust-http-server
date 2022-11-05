mod http;
mod server;

use http::method::Method;
use http::request::Request;
use server::Server;
fn main() {
    // Because we're using a mod, we must prepend the mod name to reference the method
    let server = Server::new("127.0.0.1:8080".to_string());

    server.run();
}
