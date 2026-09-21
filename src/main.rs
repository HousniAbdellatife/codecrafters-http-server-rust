mod http;
mod handlers;
mod config;

use std::io::{BufRead, Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

const CONTENT_LENGTH : &str = "Content-Length";

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
               handlers::handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}