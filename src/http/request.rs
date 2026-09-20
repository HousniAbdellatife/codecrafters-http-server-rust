use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

const CONTENT_LENGTH: &str = "Content-Length";

pub struct HttpRequest {
    pub method: String,
    pub target: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn parse(mut stream: &TcpStream) -> Result<HttpRequest, &'static str> {
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
            } else {
                let (k, v) = header.split_once(':').unwrap();
                headers.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        //

        let mut body = String::new();
        if method == "POST" {
            let content_length = headers.get(crate::CONTENT_LENGTH).unwrap().parse::<i32>().unwrap();
            let mut body_u8 = vec![0u8; content_length as usize];
            reader.read_exact(&mut body_u8).unwrap();
            body.push_str(&String::from_utf8(body_u8).unwrap());
        } else if method == "GET" {
            body.push_str("");
        }

        Ok(HttpRequest {
            method,
            target,
            headers,
            body,
        })
    }
}