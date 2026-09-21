use std::collections::HashMap;
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;
use crate::http::HttpRequest;

const HTTP_VERSION: &str = "HTTP/1.1";
const ACCEPT_ENCODING_HEADER : &str = "Accept-Encoding";
const CONTENT_ENCODING_HEADER : &str = "Content-Encoding";
const GZIP: &str = "gzip"; 

const CONTENT_LENGTH_HEADER : &str = "Content-Length";

const CRLF: &str = "\r\n";

pub struct HttpResponse {
    pub status_code: i32,
    pub reason_phrase: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status_code: i32, reason_phrase: String, request: &HttpRequest, mut response_headers: HashMap<String, String>, mut body: String) -> HttpResponse {

        let mut compressed: Vec<u8> = body.as_bytes().to_vec();

        if response_headers.contains_key(ACCEPT_ENCODING_HEADER) && !body.is_empty() {
            let gzip = response_headers.get(ACCEPT_ENCODING_HEADER).unwrap().split(',')
                .map(|s| s.trim().to_string())
                .any(|s| s == GZIP);

            if gzip {
                response_headers.insert(CONTENT_ENCODING_HEADER.to_string(), GZIP.to_string());
                let result = Self::compress(body.as_str());
                compressed = result.unwrap();
            }
        };

        HttpResponse {
            status_code,
            reason_phrase,
            headers: response_headers,
            body: compressed
        }
    }

    pub fn new_no_compression(status_code: i32, reason_phrase: String, headers: HashMap<String, String>, body: Vec<u8>) -> HttpResponse {

        HttpResponse {
            status_code,
            reason_phrase,
            headers,
            body
        }
    }

    fn compress(input: &str) -> std::io::Result<Vec<u8>> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

        encoder.write_all(input.as_bytes())?;

        encoder.finish()
    }

    pub fn build(&self) -> Vec<u8> {
        // 1. Initialize an empty byte vector instead of a String
        let mut response = Vec::new();

        // 2. Format the Status Line and extend it into the byte vector
        let status_line = format!(
            "{} {} {}{CRLF}",
            HTTP_VERSION,
            self.status_code,
            self.reason_phrase
        );
        response.extend_from_slice(status_line.as_bytes());

        // 3. Add the Headers
        for (key, val) in &self.headers {
            let header_line = format!(
                "{}: {}{CRLF}",
                key.trim(),
                val.trim()
            );
            response.extend_from_slice(header_line.as_bytes());
        }

        // 4. Always specify the exact binary body byte length
        let content_length = format!(
            "{}: {}{CRLF}",
            CONTENT_LENGTH_HEADER,
            self.body.len()
        );
        response.extend_from_slice(content_length.as_bytes());

        // 5. End the header section with the mandatory blank line boundary
        response.extend_from_slice(CRLF.as_bytes());

        // 6. Directly append the raw binary body bytes (Zero-cost, zero allocations, no panics)
        response.extend_from_slice(&self.body);

        response
    }
}