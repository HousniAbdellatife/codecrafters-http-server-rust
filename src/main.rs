use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;


const OK_200: &[u8] = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
const NOT_FOUND_404: &[u8] = "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes();


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let buf = &mut [0; 1024];
                stream.read(buf).unwrap();
                let request_line = str::from_utf8(buf)
                    .unwrap()
                    .lines()
                    .next()
                    .unwrap();

                let request_target = get_request_target(request_line);

                match request_target {
                    "/" => {stream.write_all(OK_200).unwrap();}
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

fn get_request_target(url: &str) -> &str {
    url.split(" ").collect::<Vec<&str>>().get(1).unwrap()
}
