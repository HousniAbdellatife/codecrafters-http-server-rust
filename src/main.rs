use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;


const OK_200: &[u8] = "HTTP/1.1 200 OK\r\n\r\n hello".as_bytes();

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                stream.write(OK_200).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
