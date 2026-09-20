use std::collections::HashMap;

pub const OK_200: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
pub const CREATED_201: &[u8] = b"HTTP/1.1 201 Created\r\n\r\n";
pub const NOT_FOUND_404: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";
const HTTP_VERSION: &str = "HTTP/1.1";



pub struct HttpResponse {
    pub status_code: i32,
    pub reason_phrase: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status_code: i32, reason_phrase: String, headers: HashMap<String, String>, body: String) -> HttpResponse {
        HttpResponse {
            status_code,
            reason_phrase,
            headers,
            body
        }
    }

    pub fn build(&self) -> String {
        let mut response = String::new();
        
        // status line
        let status_line = format!("{} {} {}\r\r", HTTP_VERSION, self.status_code, self.reason_phrase);
        response.push_str(&status_line);
        
        // headers
        for (key, val) in self.headers.iter() {
            response.push_str(&format!("{}: {}\r\n", key.trim(), val.trim()));
        }
        response.push_str("\r\n");
        
        // body
        if self.body.len() > 0 {
            response.push_str(self.body.as_str());
            response.push_str("\r\n");
        }
        
        response
    }
}


pub fn text_plain(content: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    )
        .into_bytes()
}

pub fn octet_stream(file_bytes: &[u8]) -> Vec<u8> {
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
        file_bytes.len()
    );

    let mut response = header.into_bytes();
    response.extend_from_slice(file_bytes);
    response
}