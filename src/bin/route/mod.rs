use httpie::srv::Request;
use httpie::srv::Response;
use httpie::srv::Content;
use httpie::srv::http;

pub fn hello_world(req: Request) -> Response {

    Response {
        body: Content::HeapString(format!(
"{{
    \"path\":{},
    \"method\":{},
    \"content-type\":{},
    \"protocol\":{},
    \"status\":{}
}}",
            req.path,
            req.method.as_str(),
            req.content_type.as_str(),
            req.protocol.as_str(),
            req.status.as_str(),
        )),
        status: http::StatusCode::Http200Ok,
        content_type: http::ContentType::ApplicationJson
    }
}

pub fn route_cwd(_req: Request) -> Response {

    const ERROR_MSG: Content = Content::StaticString(
        "Failed to get current working directory."
    );

    Response {
        body: match std::env::current_dir() {
            Ok(res) => match res.into_os_string().into_string() {
                Ok(res) => Content::HeapString(res),
                Err(_) => ERROR_MSG
            },
            Err(_) => ERROR_MSG
        },
        status: http::StatusCode::Http200Ok,
        content_type: http::ContentType::TextPlain
    }
}