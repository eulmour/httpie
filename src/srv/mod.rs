use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::pool::ThreadPool;

pub mod http;
use http::*;

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub params: Vec<(String, String)>,
    pub method: Method,
    pub protocol: Protocol,
    pub content: Vec<u8>,
    pub content_type: ContentType,
    pub content_size: usize,
    pub status: StatusCode
}
pub enum Content {
    HeapString(String),
    StaticString(&'static str),
    Raw(Vec<u8>),
    None
}
pub struct Response {
    pub body: Content,
    pub content_type: ContentType,
    pub status: StatusCode,
}

type Route = dyn Fn(Request) -> Response + Send + Sync;
type RouteMap = Arc<HashMap<&'static str, Arc<Route>>>;

#[derive(Default)]
pub struct Server {
    pub address: String,
    pub public: Arc<Option<PathBuf>>,
    pub max_connections: usize,
    pub routes: RouteMap
}

pub const RES_NOT_FOUND: Response = Response {
    body: Content::StaticString("
<!DOCTYPE html>
<html lang=\"en\">
<head><title>404 Not Found</title></head>
<body><h1>Not Found</h1>The requested URL was not found on this server.</body>
</html>"),
    status: StatusCode::Http404NotFound,
    content_type: ContentType::TextHtml
};

pub const RES_SERVER_ERROR: Response = Response {
    body: Content::StaticString("
<!DOCTYPE html>
<html lang=\"en\">
<head><title>500 Internal Server Error</title></head>
<body><h1>Iternal Server Error</h1>Yet another error to catch.</body>
</html>"),
    status: StatusCode::Http500InternalServerError,
    content_type: ContentType::TextHtml
};

impl Server {

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    
    pub fn address(mut self, address: &str) -> Self {
        self.address = String::from(address);
        self
    }

    pub fn public(mut self, path: &str) -> Self {
        self.public = Arc::new(Some(Path::new(path).to_path_buf()));
        self
    }

    pub fn max_connections(mut self, num: usize) -> Self {
        self.max_connections = num;
        self
    }

    pub fn routes(mut self, routes: RouteMap) -> Self {
        self.routes = routes;
        self
    }
    
    pub fn run(&self) -> () {

        let listener = TcpListener::bind(&self.address)
            .expect("Error: cannor bind address");
        let pool = ThreadPool::new(self.max_connections);

        for stream_res in listener.incoming() {

            let routes = Arc::clone(&self.routes);
            let public = Arc::clone(&self.public);

            pool.execute(move || {

                let mut stream = stream_res.unwrap();

                let request = Request::from(&mut stream); // TODO fix empty param bug

                let response: Response = match routes.get(request.path.as_str()) {
                    Some(r) => r(request),
                    None => match public.as_ref().as_ref() {
                        Some(val) => {

                            let resource_path = if request.path != "/" {
                                Path::new(val).join(&request.path.as_str()[1..])
                            } else {
                                Path::new(val).join("index.html")
                            };

                            let content_type_guessed = ContentType::guess(&resource_path);

                            if resource_path.is_file() {

                                match content_type_guessed {
                                    ContentType::TextHtml
                                    | ContentType::TextCss
                                    | ContentType::ApplicationJavascript
                                    | ContentType::ApplicationJson => {
                                        match std::fs::read_to_string(&resource_path) {
                                            Ok(res) => Response {
                                                body: Content::HeapString(res),
                                                status: StatusCode::Http200Ok,
                                                content_type: content_type_guessed
                                            },
                                            Err(_) => RES_SERVER_ERROR
                                        }
                                    }
                                    _ => {
                                        match std::fs::read(&resource_path) {
                                            Ok(res) => Response {
                                                body: Content::Raw(res),
                                                status: StatusCode::Http200Ok,
                                                content_type: content_type_guessed
                                            },
                                            Err(_) => RES_SERVER_ERROR
                                        }
                                    }
                                }

                            } else {
                                RES_NOT_FOUND
                            }
                        }
                        None => RES_NOT_FOUND
                    }
                };

                match &response.body {
                    Content::HeapString(string) => {
                        stream.write(format!(
                            "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
                            response.status.as_str(),
                            string.len(),
                            response.content_type.as_str(),
                            string.as_str()
                        ).as_bytes()).unwrap();
                    },
                    Content::StaticString(string) => {
                        stream.write(format!(
                            "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
                            response.status.as_str(),
                            string.len(),
                            response.content_type.as_str(),
                            *string
                        ).as_bytes()).unwrap();
                    },
                    Content::Raw(data) => {
                        stream.write(format!(
                            "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                            response.status.as_str(),
                            data.len(),
                            response.content_type.as_str(),
                        ).as_bytes()).unwrap();
                        stream.write(data).unwrap();
                    }
                    _ => ()
                };

                stream.flush().unwrap();
            });
        }
    }

}

impl Request {

    pub fn from(stream: &mut TcpStream) -> Self {

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("Error reading stream");
        let http_request_str = std::str::from_utf8(&buffer).unwrap_or_default();

        let mut req_iter = http_request_str.split_whitespace();
        let method_str = req_iter.next().unwrap_or_default();

        let query = {

            let mut query_str = req_iter
            .next()
            .unwrap_or_default()
            .split("?");

            (query_str.next().unwrap_or_default(), {
                let params = query_str.next().unwrap_or_default().split("&");

                let mut result: Vec<(String, String)> = vec![];

                for param in params {
                    let mut key_val = param.split("=");

                    result.push((
                        key_val.next().unwrap_or_default().to_owned(),
                        key_val.next().unwrap_or_default().to_owned())
                    );
                }

                result
            })
        };

        let content_size = match http_request_str.find("Content-Length: ") {
            Some(i) => {

                match http_request_str.get(i + 16..) {
                    Some(prolog) => prolog
                        .chars()
                        .take_while(|&ch| ch != '\r')
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap_or_default()
                    ,
                    None => 0
                }
            },
            None => 0
        };
        
        let content = if content_size > 0 {
            let mut content_buf: Vec<u8> = Vec::with_capacity(content_size);
            if let Err(err) = stream.take(content_size as u64).read_to_end(&mut content_buf) {
                println!("Error reading stream. {}", err);
            }
            content_buf
        } else {
            vec![]
        };

        Request {
            method: Method::from_str(method_str),
            content,
            content_type: ContentType::Unknown,
            content_size,
            protocol: Protocol::from_str(req_iter.next().unwrap_or_default()),
            status: StatusCode::Http200Ok,
            path: query.0.to_owned(),
            params: query.1
        }
    }
}