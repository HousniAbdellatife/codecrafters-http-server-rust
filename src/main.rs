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


struct HttpRequest {
    method: String,
    target: String,
    headers: HashMap<String, String>,
    body: String
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {

            Ok(mut stream) => {
                //
                thread::spawn(move || {
                    println!("accepted new connection");
                    loop {
                    let http_request = parse_http_request(&stream)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)).unwrap();

                    match http_request.target.as_str() {
                        "/" => {stream.write_all(OK_200).unwrap();}
                        e if e.starts_with("/files") => {

                            let env_args: Vec<String> = env::args().collect();
                            let dir = env_args[2].clone();

                            let file_name = e.split('/').last().unwrap();
                            let path = format!("{}/{}", dir, file_name);

                            if http_request.method == "GET" {

                                let response = return_file(path.as_str());

                                stream.write_all(&response).unwrap();
                            }else if http_request.method == "POST" {
                                create_file(path, http_request.body.as_bytes());
                                stream.write_all(CREATED_201).unwrap();
                            }

                        }
                        e if e.starts_with("/user-agent") => {
                            stream.write_all(
                                user_agent_response(http_request.headers.get("User-Agent").unwrap()).as_bytes()
                            ).expect("panic");
                        }
                        e if e.starts_with("/echo") => {stream.write_all(http_echo(e).as_bytes()).unwrap();}
                        _ => {stream.write_all(NOT_FOUND_404).unwrap();}
                    }}
                });

                //
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn create_file(path: String, content: &[u8]) {
    let path = Path::new(&path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).expect("panic to write the file");
}

fn return_file(path: &str) -> Vec<u8> {
    let file_exists = fs::exists(path).unwrap();
    if !file_exists {
        return NOT_FOUND_404.to_vec();
    }

    let file = fs::read(path).unwrap();
    let mut response = String::new();
    response.push_str("HTTP/1.1 200 OK\r\n");
    response.push_str("Content-Type: application/octet-stream\r\n");
    response.push_str(format!("Content-Length: {}\r\n\r\n", file.len()).as_str());
    response.push_str(String::from_utf8(file).unwrap().as_str());
    println!("response: {}", response);
    response.into_bytes()
}
fn user_agent_response(text: &String) -> String{
    format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        text.len(),
        text
    )
}

fn http_echo(request_line: &str) -> String {
    let text = request_line
        .split('/')
        .skip(2)
        .next()
        .unwrap().trim();

    format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        text.len(),
        text
    )
}

fn parse_http_request(mut stream: &TcpStream) -> Result<HttpRequest, &'static str> {


    let mut request_line = String::new();
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut body = String::new();

    let mut reader = BufReader::new(&mut stream);

    //
    reader.read_line(&mut request_line).expect("TODO: panic message");
    let mut request_line = request_line.split_whitespace();
    let method = request_line.next().unwrap().to_string();
    let target = request_line.next().unwrap().to_string();
    let http_version = request_line.next().unwrap().to_string();

    //
    let mut header = String::new();
    loop {
        header.clear();
        reader.read_line(&mut header).unwrap();
        if header.is_empty() || header == "\r\n" {
            break;
        }else {
            let (k, v) = header.split_once(':').unwrap();
            headers.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    //

    let mut body = String::new();
    if method == "POST" {
        let content_length = headers.get(CONTENT_LENGTH).unwrap().parse::<i32>().unwrap();
        let mut body_u8 = vec![0u8; content_length as usize];
        reader.read_exact(&mut body_u8).unwrap();
        body.push_str(&String::from_utf8(body_u8).unwrap());
    }else if method == "GET" {
        body.push_str("");
    }

    Ok(HttpRequest {
        method,
        target,
        headers,
        body,
    })
}