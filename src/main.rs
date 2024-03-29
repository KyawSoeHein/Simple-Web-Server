#![allow(dead_code)]

mod http;
mod server;

use http::Request;
use server::Server;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}
