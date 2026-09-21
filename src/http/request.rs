use std::collections::HashMap;
use std::io::BufRead;


const GET: &str = "GET";
const POST: &str = "POST";

const CRLF: &str = "\r\n";


pub struct HttpRequest {
    pub method: String,
    pub target: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn parse_next_request(reader: &mut impl BufRead) -> Result<HttpRequest, &'static str> {
        let mut request_line = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();

        // read first line
        if reader
            .read_line(&mut request_line)
            .map_err(|_| "failed to read request line")?
            == 0
        {
            return Err("connection closed");
        }
        let mut request_line = request_line.split_whitespace();
        let method = request_line.next().unwrap().to_string();
        let target = request_line.next().unwrap().to_string();

        // read headers
        let mut header = String::new();
        loop {
            header.clear();
            reader.read_line(&mut header).unwrap();
            if header.is_empty() || header == CRLF {
                break;
            } else {
                let (k, v) = header.split_once(':').unwrap();
                headers.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        // read body
        let mut body = String::new();
        if method == POST {
            let content_length = headers.get(crate::CONTENT_LENGTH).unwrap().parse::<i32>().unwrap();
            let mut body_u8 = vec![0u8; content_length as usize];
            reader.read_exact(&mut body_u8).unwrap();
            body.push_str(&String::from_utf8(body_u8).unwrap());
        } else if method == GET {
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

#[cfg(test)]
mod tests {
    use std::fmt::format;
    use std::io::{BufReader, Cursor};

    use super::{HttpRequest, CRLF};

    #[test]
    fn parses_two_requests_from_one_buffered_connection() {
        let input = format!("GET / HTTP/1.1{CRLF}{CRLF}GET /echo/hello HTTP/1.1{CRLF}{CRLF}");
        let mut reader = BufReader::new(Cursor::new(input));

        let first = HttpRequest::parse_next_request(&mut reader).unwrap();
        let second = HttpRequest::parse_next_request(&mut reader).unwrap();

        assert_eq!(first.target, "/");
        assert_eq!(second.target, "/echo/hello");
    }
}
