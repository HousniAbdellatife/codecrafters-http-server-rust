use std::collections::HashMap;
use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;

const OK_200: &[u8] = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
const NOT_FOUND_404: &[u8] = "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes();


struct HttpRequest {
    method: String,
    path: String,
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
                println!("accepted new connection");
                let http_request = parse_http_request(&stream)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)).unwrap();

                match http_request.path.as_str() {
                    "/" => {stream.write_all(OK_200).unwrap();}
                    e if e.starts_with("/user-agent") => {
                            stream.write_all(
                                user_agent_response(http_request.headers.get("User-Agent").unwrap()).as_bytes()
                            ).expect("panic");
                         }
                    e if e.starts_with("/echo") => {stream.write_all(http_echo(e).as_bytes()).unwrap();}
                    _ => {stream.write_all(NOT_FOUND_404).unwrap();}
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
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


fn read_stream(mut stream: &TcpStream) -> Result<String, &'static str> {

    let mut buffer = [0; 1024];
    let mut request: Vec<u8> = Vec::new();

    loop {
        let n = stream.read(&mut buffer)
            .map_err(|_| "failed to read from tcp stream")?;

        if n == 0 {
            break;
        }

        request.extend_from_slice(&buffer[..n]);

        if request.windows(4).any(|x| x == b"\r\n\r\n") {
            break;
        }
    }

    Ok(String::from_utf8(request)
        .map_err(|_| "failed to transform into UTF-8")?)
}

fn parse_http_request(mut stream: &TcpStream) -> Result<HttpRequest, &'static str> {

    let request = read_stream(&mut stream)?;
    let mut lines = request.lines();

    // first part
    let mut request_line = lines.next().unwrap().split_whitespace();
    let method = request_line.next().unwrap().to_string();
    let target = request_line.next().unwrap().to_string();
    let http_version = request_line.next().unwrap().to_string();

    //  headers
    let mut headers: HashMap<String, String> = HashMap::new();
    for line in lines {
        if line.is_empty() {break;}
        let (key, value) = line.split_once(':').unwrap();
        headers.insert(key.trim().to_string(), value.trim().to_string());
    }

    // body

    Ok(HttpRequest {
        method: method,
        path: target,
        headers: headers,
        body: "".to_string(),
    })
}