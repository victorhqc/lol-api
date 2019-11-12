mod summoner;
pub use self::summoner::*;

use crate::regions::WithHosts;
use failure::Error;
use hyper::client::HttpConnector;
use hyper::header::HeaderValue;
use hyper::rt::Future;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;

pub struct Api<T> {
    api_host: String,
    api_key: String,
    client: HttpsClient,
    platform: T,
}

impl<T> Api<T>
where
    T: WithHosts,
{
    pub fn new(api_key: String, platform: T, api_host: Option<String>) -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let api_host = match api_host {
            Some(h) => h,
            None => String::from("api.riotgames.com"),
        };

        Self {
            api_host,
            api_key,
            platform,
            client,
        }
    }

    pub fn build_request(&self, method: Method, path: String) -> Result<Request<Body>> {
        let uri = self.get_uri(path);
        let mut req = Request::new(Body::empty());
        *req.method_mut() = method;
        *req.uri_mut() = uri;

        req.headers_mut()
            .insert("X-Riot-Token", HeaderValue::from_str(&self.api_key)?);

        Ok(req)
    }

    fn get_uri(&self, path: String) -> Uri {
        println!("https://{}{}", self.platform.host(&self.api_host), path);
        format!("https://{}{}", self.platform.host(&self.api_host), path)
            .parse::<Uri>()
            .unwrap()
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type FutureResult<T> = dyn Future<Item = T, Error = Error>;
pub type HttpsClient = Client<HttpsConnector<HttpConnector>>;
