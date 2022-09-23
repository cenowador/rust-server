#![allow(dead_code)]
use server::Server;
use webserver_handler::WebserverHandler;
use std::env;

mod server;
mod http;
mod webserver_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());

    println!("public path: {}", public_path);
    server.run(WebserverHandler::new(public_path));
}
