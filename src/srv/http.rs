#[derive(Debug)]
pub enum Method {
    Unknown,
    Get,
    Post,
    Put
}

#[derive(Debug)]
pub enum Protocol {
    Unknown,
    V10,
    V11,
    V20,
    V30
}

#[derive(Debug)]
pub enum ContentType {
    Unknown,
    TextPlain,
    TextHtml,
    TextCss,
    ImagePng,
    ImageJpeg,
    ImageWebp,
    ImageIcon,
    ApplicationJavascript,
    ApplicationJson,
    ApplicationWasm,
    ApplicationXml,
    AudioAac,
    AudioMpeg,
    AudioOgg,
    AudioWebm,
    VideoMpeg,
    VideoMp4,
    VideoWebm,
}

#[derive(Debug)]
pub enum StatusCode {
    Unknown,
    Http100Continue, // info
    Http101SwitchingProtocols,
    Http102Processing,
    Http103EarlyHints,
    Http200Ok, // success
    Http201Created,
    Http202Accepted,
    Http203NonAuthoritativeInformation,
    Http204NoContent,
    Http205ResetContent,
    Http206PartialContent,
    Http207MultiStatus,
    Http208AlreadyReported,
    Http226ImUsed,
    Http300MultipleChoices, // redirect
    Http301MovedPermanently,
    Http302MovedTemporarily,
    Http303SeeOther,
    Http304NotModified,
    Http305UseProxy,
    Http306Reserved,
    Http307TemporaryRedirect,
    Http308PermanentRedirect,
    Http400BadRequest, // client error
    Http401Unauthorized,
    Http402PaymentRequired,
    Http403Forbidden,
    Http404NotFound,
    Http405MethodNotAllowed,
    Http406NotAcceptable,
    Http407ProxyAuthenticationRequired,
    Http408RequestTimeout,
    Http409Conflict,
    Http410Gone,
    Http411LengthRequired,
    Http412PreconditionFailed,
    Http413PayloadToolarge,
    Http414UriTooLong,
    Http415UnsupportedMediaType,
    Http416RangeNotSatisfiable,
    Http417ExpectationFailed,
    Http418IAmATeapot,
    Http419AuthenticationTimeout,
    Http421MisdirectedRequest,
    Http422UnprocessableEntity,
    Http423Locked,
    Http424FailedDependency,
    Http425TooEarly,
    Http426UpgradeRequired,
    Http428PreconditionRequired,
    Http429TooManyRequests,
    Http431RequestHeaderFieldsTooLarge,
    Http449RetryWith,
    Http451UnavailableForLegalReasons,
    Http499ClientClosedRequest,
    Http500InternalServerError, // server error
    Http501NotImplemented,
    Http502BadGateway,
    Http503ServiceUnavailable,
    Http504GatewayTimeout,
    Http505HttpVersionNotSupported,
    Http506VariantAlsoNegotiates,
    Http507InsufficientStorage,
    Http508LoopDetected,
    Http509BandwidthLimitExceeded,
    Http510NotExtended,
    Http511NetworkAuthenticationRequired,
    Http520UnknownError,
    Http521WebServerIsDown,
    Http522ConnectionTimedOut,
    Http523OriginIsUnreachable,
    Http524ATimeoutOccurred,
    Http525SslHandshakeFailed,
    Http526InvalidSslCertificate
}

impl Method {
    // pub const VALUES: [Self; 5] = [Self::Add, Self::ConfigFile, Self::Label, Self::Print, Self::Verbose];
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Unknown => "Unknown",
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT"
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "Unknown" | _ => Method::Unknown
        }
    }
}

impl Protocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Protocol::Unknown => "Unknown",
            Protocol::V10 => "HTTP 1.0",
            Protocol::V11 => "HTTP 1.1",
            Protocol::V20 => "HTTP 2.0",
            Protocol::V30 => "HTTP 3.0",

        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "HTTP 1.0" => Protocol::V10,
            "HTTP 1.1" => Protocol::V11,
            "HTTP 2.0" => Protocol::V20,
            "HTTP 3.0" => Protocol::V30,
            "Unknown" | _ => Protocol::Unknown,
        }
    }
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Unknown => "*/*",
            ContentType::TextPlain => "text/plain",
            ContentType::TextHtml => "text/html",
            ContentType::TextCss => "text/css",
            ContentType::ApplicationJavascript => "application/javascript",
            ContentType::ApplicationJson => "application/json",
            ContentType::ApplicationWasm => "application/wasm",
            ContentType::ApplicationXml => "application/xml",
            ContentType::ImagePng => "image/png",
            ContentType::ImageJpeg => "image/jpeg",
            ContentType::ImageIcon => "image/vnd.microsoft.icon",
            ContentType::ImageWebp => "image/webp",
            ContentType::AudioAac => "audio/aac",
            ContentType::AudioMpeg => "audio/mpeg",
            ContentType::AudioOgg => "audio/ogg",
            ContentType::AudioWebm => "video/webm",
            ContentType::VideoMpeg => "video/mpeg",
            ContentType::VideoMp4 => "video/mp4",
            ContentType::VideoWebm => "video/webm",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "text/html" => ContentType::TextHtml,
            "text/css" => ContentType::TextCss,
            "text/plain" => ContentType::TextPlain,
            "application/javascript" => ContentType::ApplicationJavascript,
            "application/json" => ContentType::ApplicationJson,
            "application/wasm" => ContentType::ApplicationWasm,
            "application/xml" => ContentType::ApplicationXml,
            "text/javascript" => ContentType::ApplicationJavascript,
            "text/xml" => ContentType::ApplicationXml,
            "image/jpeg" => ContentType::ImageJpeg,
            "image/png" => ContentType::ImagePng,
            "image/ico" => ContentType::ImageIcon,
            "image/webp" => ContentType::ImageWebp,
            "audio/aac" => ContentType::AudioAac,
            "audio/mpeg" => ContentType::AudioMpeg,
            "audio/ogg" => ContentType::AudioOgg,
            "video/mpeg" => ContentType::VideoMpeg,
            "video/mp4" => ContentType::VideoMp4,
            "video/webm" => ContentType::AudioWebm,
            "*/*" | _ => ContentType::Unknown,
        }
    }

    pub fn guess(path: &std::path::Path) -> Self {

        match path.extension() {

            Some(val) => {

                if let Some(ext_str) = val.to_str() {
                    match ext_str {
                        "html" => ContentType::TextHtml,
                        "css" => ContentType::TextCss,
                        "js" => ContentType::ApplicationJavascript,
                        "png" => ContentType::ImagePng,
                        "json" => ContentType::ApplicationJson,
                        "ico" => ContentType::ImageIcon,
                        "wasm" => ContentType::ApplicationWasm,
                        "txt" => ContentType::TextPlain,
                        "xml" => ContentType::ApplicationXml,
                        "jpg" => ContentType::ImageJpeg,
                        "webp" => ContentType::ImageWebp,
                        "aac" => ContentType::AudioAac,
                        "mp3" => ContentType::AudioMpeg,
                        "ogg" => ContentType::AudioOgg,
                        "mpeg" => ContentType::VideoMpeg,
                        "mp4" => ContentType::VideoMp4,
                        "webm" => ContentType::AudioWebm,
                        "*/*" | _ => ContentType::Unknown,
                    }
                } else {
                    return ContentType::Unknown;
                }
            },

            None => ContentType::Unknown
        }
    }

}

impl StatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusCode::Unknown => "Unknown",
            StatusCode::Http100Continue => "100 Continue",
            StatusCode::Http101SwitchingProtocols => "101 Switching Protocols",
            StatusCode::Http102Processing => "102 Processing",
            StatusCode::Http103EarlyHints => "103 Early Hints",
            StatusCode::Http200Ok => "200 OK",
            StatusCode::Http201Created => "201 Created",
            StatusCode::Http202Accepted => "202 Accepted",
            StatusCode::Http203NonAuthoritativeInformation => "203 Non-Authoritative Information",
            StatusCode::Http204NoContent => "204 No Content",
            StatusCode::Http205ResetContent => "205 Reset Content",
            StatusCode::Http206PartialContent => "206 Partial Content",
            StatusCode::Http207MultiStatus => "207 Multi-Status",
            StatusCode::Http208AlreadyReported => "208 Already Reported",
            StatusCode::Http226ImUsed => "226 Im Used",
            StatusCode::Http300MultipleChoices => "300 Multiple Choices",
            StatusCode::Http301MovedPermanently => "301 Moved Permanently",
            StatusCode::Http302MovedTemporarily => "302 Moved Temporarily",
            StatusCode::Http303SeeOther => "303 See Other",
            StatusCode::Http304NotModified => "304 Not Modified",
            StatusCode::Http305UseProxy => "305 Use Proxy",
            StatusCode::Http306Reserved => "306 Reserved",
            StatusCode::Http307TemporaryRedirect => "307 Temporary Redirect",
            StatusCode::Http308PermanentRedirect => "308 Permanent Redirect",
            StatusCode::Http400BadRequest => "400 Bad Request",
            StatusCode::Http401Unauthorized => "401 Unauthorized",
            StatusCode::Http402PaymentRequired => "402 Payment Required",
            StatusCode::Http403Forbidden => "403 Forbidden",
            StatusCode::Http404NotFound => "404 Not Found",
            StatusCode::Http405MethodNotAllowed => "405 Method Not Allowed",
            StatusCode::Http406NotAcceptable => "406 Not Acceptable",
            StatusCode::Http407ProxyAuthenticationRequired => "407 Proxy Authentication Required",
            StatusCode::Http408RequestTimeout => "408 Request Timeout",
            StatusCode::Http409Conflict => "409 Conflict",
            StatusCode::Http410Gone => "410 Gone",
            StatusCode::Http411LengthRequired => "411 Length Required",
            StatusCode::Http412PreconditionFailed => "412 Precondition Failed",
            StatusCode::Http413PayloadToolarge => "413 Payload Too Large",
            StatusCode::Http414UriTooLong => "414 URL Too Long",
            StatusCode::Http415UnsupportedMediaType => "415 Unsupported Media Type",
            StatusCode::Http416RangeNotSatisfiable => "416 Range Not Satisfiable",
            StatusCode::Http417ExpectationFailed => "417 Expectation Failed",
            StatusCode::Http418IAmATeapot => "418 I Am A Teapot",
            StatusCode::Http419AuthenticationTimeout => "419 Authentication Timeout",
            StatusCode::Http421MisdirectedRequest => "421 Misdirected Request",
            StatusCode::Http422UnprocessableEntity => "422 Uprocessable Entity",
            StatusCode::Http423Locked => "423 Locked",
            StatusCode::Http424FailedDependency => "424 Failed Dependency",
            StatusCode::Http425TooEarly => "425 Too Early",
            StatusCode::Http426UpgradeRequired => "426 Upgrade Required",
            StatusCode::Http428PreconditionRequired => "428 Precondition Required",
            StatusCode::Http429TooManyRequests => "429 Too Many Requests",
            StatusCode::Http431RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large",
            StatusCode::Http449RetryWith => "449 Retry With",
            StatusCode::Http451UnavailableForLegalReasons => "451 Unavailable For Legal Reasons",
            StatusCode::Http499ClientClosedRequest => "499 Client Closed Request",
            StatusCode::Http500InternalServerError => "500 Internal Server Error",
            StatusCode::Http501NotImplemented => "501 Not Implemented",
            StatusCode::Http502BadGateway => "502 Bad Gateway",
            StatusCode::Http503ServiceUnavailable => "503 Service Unavailable",
            StatusCode::Http504GatewayTimeout => "504 Gateway Timeout",
            StatusCode::Http505HttpVersionNotSupported => "505 HTTP Version Not Supported",
            StatusCode::Http506VariantAlsoNegotiates => "506 Variant Also Negotiated",
            StatusCode::Http507InsufficientStorage => "507 Insufficient Storage",
            StatusCode::Http508LoopDetected => "508 Loop Detected",
            StatusCode::Http509BandwidthLimitExceeded => "509 Bandwidth Limit Exceeded",
            StatusCode::Http510NotExtended => "510 Not Extended",
            StatusCode::Http511NetworkAuthenticationRequired => "511 Network Authentication Required",
            StatusCode::Http520UnknownError => "520 Unknown Error",
            StatusCode::Http521WebServerIsDown => "521 Web Server Is Down",
            StatusCode::Http522ConnectionTimedOut => "522 Connection Timed Out",
            StatusCode::Http523OriginIsUnreachable => "523 Origin Is Unreachable",
            StatusCode::Http524ATimeoutOccurred => "524 A Timeout Occurred",
            StatusCode::Http525SslHandshakeFailed => "525 SSL Handshake Failed",
            StatusCode::Http526InvalidSslCertificate => "526 Invalid SSL Certificate"
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "100 Continue" => StatusCode::Http100Continue,
            "101 Switching Protocols" => StatusCode::Http101SwitchingProtocols,
            "102 Processing" => StatusCode::Http102Processing,
            "103 Early Hints" => StatusCode::Http103EarlyHints,
            "200 OK" => StatusCode::Http200Ok,
            "201 Created" => StatusCode::Http201Created,
            "202 Accepted" => StatusCode::Http202Accepted,
            "203 Non-Authoritative Information" => StatusCode::Http203NonAuthoritativeInformation,
            "204 No Content" => StatusCode::Http204NoContent,
            "205 Reset Content" => StatusCode::Http205ResetContent,
            "206 Partial Content" => StatusCode::Http206PartialContent,
            "207 Multi-Status" => StatusCode::Http207MultiStatus,
            "208 Already Reported" => StatusCode::Http208AlreadyReported,
            "226 Im Used" => StatusCode::Http226ImUsed,
            "300 Multiple Choices" => StatusCode::Http300MultipleChoices,
            "301 Moved Permanently" => StatusCode::Http301MovedPermanently,
            "302 Moved Temporarily" => StatusCode::Http302MovedTemporarily,
            "303 See Other" => StatusCode::Http303SeeOther,
            "304 Not Modified" => StatusCode::Http304NotModified,
            "305 Use Proxy" => StatusCode::Http305UseProxy,
            "306 Reserved" => StatusCode::Http306Reserved,
            "307 Temporary Redirect" => StatusCode::Http307TemporaryRedirect,
            "308 Permanent Redirect" => StatusCode::Http308PermanentRedirect,
            "400 Bad Request" => StatusCode::Http400BadRequest,
            "401 Unauthorized" => StatusCode::Http401Unauthorized,
            "402 Payment Required" => StatusCode::Http402PaymentRequired,
            "403 Forbidden" => StatusCode::Http403Forbidden,
            "404 Not Found" => StatusCode::Http404NotFound,
            "405 Method Not Allowed" => StatusCode::Http405MethodNotAllowed,
            "406 Not Acceptable" => StatusCode::Http406NotAcceptable,
            "407 Proxy Authentication Required" => StatusCode::Http407ProxyAuthenticationRequired,
            "408 Request Timeout" => StatusCode::Http408RequestTimeout,
            "409 Conflict" => StatusCode::Http409Conflict,
            "410 Gone" => StatusCode::Http410Gone,
            "411 Length Required" => StatusCode::Http411LengthRequired,
            "412 Precondition Failed" => StatusCode::Http412PreconditionFailed,
            "413 Payload Too Large" => StatusCode::Http413PayloadToolarge,
            "414 URL Too Long" => StatusCode::Http414UriTooLong,
            "415 Unsupported Media Type" => StatusCode::Http415UnsupportedMediaType,
            "416 Range Not Satisfiable" => StatusCode::Http416RangeNotSatisfiable,
            "417 Expectation Failed" => StatusCode::Http417ExpectationFailed,
            "418 I Am A Teapot" => StatusCode::Http418IAmATeapot,
            "419 Authentication Timeout" => StatusCode::Http419AuthenticationTimeout,
            "421 Misdirected Request" => StatusCode::Http421MisdirectedRequest,
            "422 Uprocessable Entity" => StatusCode::Http422UnprocessableEntity,
            "423 Locked" => StatusCode::Http423Locked,
            "424 Failed Dependency" => StatusCode::Http424FailedDependency,
            "425 Too Early" => StatusCode::Http425TooEarly,
            "426 Upgrade Required" => StatusCode::Http426UpgradeRequired,
            "428 Precondition Required" => StatusCode::Http428PreconditionRequired,
            "429 Too Many Requests" => StatusCode::Http429TooManyRequests,
            "431 Request Header Fields Too Large" => StatusCode::Http431RequestHeaderFieldsTooLarge,
            "449 Retry With" => StatusCode::Http449RetryWith,
            "451 Unavailable For Legal Reasons" => StatusCode::Http451UnavailableForLegalReasons,
            "499 Client Closed Request" => StatusCode::Http499ClientClosedRequest,
            "500 Internal Server Error" => StatusCode::Http500InternalServerError,
            "501 Not Implemented" => StatusCode::Http501NotImplemented,
            "502 Bad Gateway" => StatusCode::Http502BadGateway,
            "503 Service Unavailable" => StatusCode::Http503ServiceUnavailable,
            "504 Gateway Timeout" => StatusCode::Http504GatewayTimeout,
            "505 HTTP Version Not Supported" => StatusCode::Http505HttpVersionNotSupported,
            "506 Variant Also Negotiated" => StatusCode::Http506VariantAlsoNegotiates,
            "507 Insufficient Storage" => StatusCode::Http507InsufficientStorage,
            "508 Loop Detected" => StatusCode::Http508LoopDetected,
            "509 Bandwidth Limit Exceeded" => StatusCode::Http509BandwidthLimitExceeded,
            "510 Not Extended" => StatusCode::Http510NotExtended,
            "511 Network Authentication Required" => StatusCode::Http511NetworkAuthenticationRequired,
            "520 Unknown Error" => StatusCode::Http520UnknownError,
            "521 Web Server Is Down" => StatusCode::Http521WebServerIsDown,
            "522 Connection Timed Out" => StatusCode::Http522ConnectionTimedOut,
            "523 Origin Is Unreachable" => StatusCode::Http523OriginIsUnreachable,
            "524 A Timeout Occurred" => StatusCode::Http524ATimeoutOccurred,
            "525 SSL Handshake Failed" => StatusCode::Http525SslHandshakeFailed,
            "526 Invalid SSL Certificate" => StatusCode::Http526InvalidSslCertificate,
            "Unknown" | _ => StatusCode::Unknown
        }
    }
}

