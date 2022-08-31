#![allow(dead_code)]
mod server;
mod http;

use http::Request;
use http::Method;
use server::Server;
fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}



