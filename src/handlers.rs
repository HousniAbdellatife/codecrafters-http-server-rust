use std::{env, fs, thread};
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use crate::config::Config;
use crate::http::{HttpRequest};
use crate::http::response::HttpResponse;

const ROOT_TARGET: &str = "/";
const FILES_TARGET: &str = "/files";

const USER_AGENT_TARGET: &str = "/user-agent";

const ECHO_TARGET: &str = "/echo";

pub fn handle_connection(mut stream: TcpStream){
    thread::spawn(move || {
        loop {
            let mut http_request = HttpRequest::parse(&stream)
                .unwrap();

            let target = http_request.target.as_str();

            let mut response = match target {
                ROOT_TARGET => handle_root_target(),
                e if target.starts_with(FILES_TARGET) => handle_files_target(&http_request),
                e if target.starts_with(USER_AGENT_TARGET) => handle_user_agent_target(&http_request),
                e if target.starts_with(ECHO_TARGET) => handle_echo_target(&http_request),
                _ => no_handlers_found()
            };

            let mut close = false;
           if http_request.headers.get("Connection").or(Some(&String::from("None"))).unwrap().to_string() == "close" {
                response.headers.insert("Connection".to_string(), String::from("close"));
                close = true;
           }

            if http_request.headers.contains_key("Accept-Encoding") {
                let gzip = http_request.headers.get("Accept-Encoding").unwrap().split(',')
                    .map(|s| s.trim().to_string())
                    .any(|s| s == "gzip");

                if gzip {response.headers.insert("Content-Encoding".to_string(), "gzip".to_string());}
            }

            stream.write_all(&response.build()).unwrap();

            if close { break }
        }
    });
}

fn handle_files_target(http_request: &HttpRequest) -> HttpResponse {
    let config = Config::parse();

    let file_name = http_request.target.split('/').last().unwrap();
    let path = format!("{}/{}", config.files_dir.as_str(), file_name);

    //  TODO: all, post, get
    match http_request.method.as_str() {
        "GET" => load_file(path.as_str()),
        _ => create_file(http_request, path.as_str())
    }
}

fn create_file(http_request: &HttpRequest, path: &str) -> HttpResponse {
    let path = Path::new(&path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, http_request.body.as_bytes()).expect("panic to write the file");


    HttpResponse::new(201, "Created".to_string(),http_request, HashMap::new(), "".to_string())

}
fn load_file(path: &str) -> HttpResponse {
    let file_exists = fs::exists(path).unwrap();
    if !file_exists {
        return HttpResponse::new_no_compression(404, String::from("Not Found"), HashMap::new(), String::from("").into_bytes())
    }

    let file = fs::read(path).unwrap();

    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
    headers.insert("Content-Length".to_string(), file.len().to_string());


    HttpResponse::new_no_compression(200, "OK".to_string(), headers, file)
}

fn handle_echo_target(request: &HttpRequest) -> HttpResponse {

    let content = request.target
        .split('/')
        .skip(2)
        .next()
        .unwrap().trim();

    let mut reponse_headers: HashMap<String, String> = HashMap::new();
    reponse_headers.insert("Content-Type".to_string(), "text/plain".to_string());
    reponse_headers.insert("Content-Length".to_string(), content.len().to_string());
    if request.headers.contains_key("Accept-Encoding") {
        reponse_headers.insert("Accept-Encoding".to_string(), request.headers["Accept-Encoding"].to_string());
    }


    HttpResponse::new(200, "OK".to_string(), request, reponse_headers, content.to_string())
}

fn handle_user_agent_target(http_request: &HttpRequest) -> HttpResponse {
    let user_agent_header = http_request.headers.get("User-Agent");

    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    headers.insert("Content-Length".to_string(), user_agent_header.unwrap().len().to_string());

    HttpResponse::new(200, "OK".to_string(), http_request, headers, user_agent_header.unwrap().trim().to_string())
}
fn no_handlers_found() -> HttpResponse {
    HttpResponse::new_no_compression(404, String::from("Not Found"), HashMap::new(), String::from("").into_bytes())
}
fn  handle_root_target() -> HttpResponse {

    HttpResponse::new_no_compression(200, String::from("OK"), HashMap::new(), String::from("").into_bytes())
}