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
