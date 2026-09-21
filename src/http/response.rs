use std::collections::HashMap;
use std::fmt::format;
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;

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
    pub fn new(status_code: i32, reason_phrase: String, mut headers: HashMap<String, String>, mut body: String) -> HttpResponse {

        if headers.contains_key("Accept-Encoding") && !body.is_empty() {
            let gzip = headers.get("Accept-Encoding").unwrap().split(',')
                .map(|s| s.trim().to_string())
                .any(|s| s == "gzip");

            if gzip {
                headers.insert("Content-Encoding".to_string(), "gzip".to_string());
                let result = Self::compress(body.as_str());
                body = String::from_utf8(result.unwrap()).unwrap().to_string()
            }
        };

        HttpResponse {
            status_code,
            reason_phrase,
            headers,
            body: String::from_utf8(Self::compress(body.as_str()).unwrap()).unwrap(),
        }
    }

    pub fn compress(input: &str) -> std::io::Result<Vec<u8>> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

        encoder.write_all(input.as_bytes())?;

        encoder.finish()
    }

    pub fn build(&self) -> String {
        let mut response = String::new();

        response.push_str(&format!(
            "{} {} {}\r\n",
            HTTP_VERSION,
            self.status_code,
            self.reason_phrase
        ));

        for (key, val) in &self.headers {
            response.push_str(&format!(
                "{}: {}\r\n",
                key.trim(),
                val.trim()
            ));
        }

        // Always specify body length
        response.push_str(&format!(
            "Content-Length: {}\r\n",
            self.body.len()
        ));

        // End headers
        response.push_str("\r\n");

        // Body, possibly empty
        response.push_str(&self.body);

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