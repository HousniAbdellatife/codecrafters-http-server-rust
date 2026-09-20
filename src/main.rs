mod http;
mod handlers;
mod config;

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;
use std::{env, fs, thread};
use std::path::Path;

const OK_200: &[u8] = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
const CREATED_201: &[u8] = b"HTTP/1.1 201 Created\r\n\r\n";
const NOT_FOUND_404: &[u8] = "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes();

const CONTENT_LENGTH : &str = "Content-Length";

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
               handlers::handle_connection(stream);
                     
                // if http_request.headers.get("Connection").or(Some(&String::from("None"))).unwrap() == "close" {
                //     println!("Closing connection");
                //     break; }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


// 
// 
// 
// fn create_file(path: String, content: &[u8]) {
//     let path = Path::new(&path);
//     fs::create_dir_all(path.parent().unwrap()).unwrap();
//     fs::write(path, content).expect("panic to write the file");
// }
// 
// fn return_file(path: &str) -> Vec<u8> {
//     let file_exists = fs::exists(path).unwrap();
//     if !file_exists {
//         return NOT_FOUND_404.to_vec();
//     }
// 
//     let file = fs::read(path).unwrap();
//     let mut response = String::new();
//     response.push_str("HTTP/1.1 200 OK\r\n");
//     response.push_str("Content-Type: application/octet-stream\r\n");
//     response.push_str(format!("Content-Length: {}\r\n\r\n", file.len()).as_str());
//     response.push_str(String::from_utf8(file).unwrap().as_str());
//     println!("response: {}", response);
//     response.into_bytes()
// }
// fn user_agent_response(text: &String) -> String{
//     format!(
//         "HTTP/1.1 200 OK\r\n\
//          Content-Type: text/plain\r\n\
//          Content-Length: {}\r\n\
//          \r\n\
//          {}",
//         text.len(),
//         text
//     )
// }
// 
// fn http_echo(request_line: &str) -> String {
//     let text = request_line
//         .split('/')
//         .skip(2)
//         .next()
//         .unwrap().trim();
// 
//     format!(
//         "HTTP/1.1 200 OK\r\n\
//          Content-Type: text/plain\r\n\
//          Content-Length: {}\r\n\
//          \r\n\
//          {}",
//         text.len(),
//         text
//     )
// }