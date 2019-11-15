use hyper::http::StatusCode;

#[derive(Fail, Debug)]
pub enum FetchError {
    #[fail(display = "HTTP Error: {}", _0)]
    Http(hyper::Error),
    #[fail(display = "JSON Deserialization Error: {}", _0)]
    Json(serde_json::Error),
    #[fail(display = "HTTP Status code: {}", _0)]
    Status(StatusCode),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

// #[derive(Fail, Debug)]
// pub enum RiotApiError {
//     #[fail(display = "Bad request: {}", _0)]
//     BadRequest(String),
//     #[fail(display = "Unauthorized: {}", _0)]
//     Unauthorized(String),
//     #[fail(display = "Forbidden: {}", _0)]
//     Forbidden(String),
//     #[fail(display = "Data not found: {}", _0)]
//     DataNotFound(String),
//     #[fail(display = "Unsupported media type: {}", _0)]
//     UnsupportedMediaType(String),
//     #[fail(display = "Rate limit exceeded: {}", _0)]
//     RateLimitExceeded(String),
//     #[fail(display = "Internal server error: {}", _0)]
//     InternalServerError(String),
//     #[fail(display = "Bad gateway: {}", _0)]
//     BadWateway(String),
//     #[fail(display = "Service unavailable: {}", _0)]
//     ServiceUnavailable(String),
//     #[fail(display = "Gateway timeout: {}", _0)]
//     GatewayTimeout(String),
// }

// pub type Result<T> = std::result::Result<T, RiotError>;
//
// #[derive(Debug)]
// pub enum RiotError {
//     Http(hyper::Error),
//     Json(serde_json::Error),
// }
//
// impl From<hyper::Error> for RiotError {
//     fn from(err: hyper::Error) -> RiotError {
//         RiotError::Http(err)
//     }
// }
//
// impl From<serde_json::Error> for RiotError {
//     fn from(err: serde_json::Error) -> RiotError {
//         RiotError::Json(err)
//     }
// }
//
// impl fmt::Display for RiotError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:#?}", "Something went wrong!")
//     }
// }
//
// impl error::Error for RiotError {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         None
//     }
// }
