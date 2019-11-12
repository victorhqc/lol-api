mod summoner;
pub use self::summoner::*;

use crate::regions::WithHosts;
use failure::Error;
use hyper::client::HttpConnector;
use hyper::header::HeaderValue;
use hyper::rt::Future;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use log::{debug};

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

        debug!("API HOST: {}", api_host);

        Self {
            api_host,
            api_key,
            platform,
            client,
        }
    }

    pub fn summoners(&self) -> SummonerApi<T> {
        let api = self.clone();
        SummonerApi::new(api)
    }

    pub fn build_request(&self, method: Method, path: String) -> Result<Request<Body>> {
        let uri = self.get_uri(path);
        debug!("{}: {}", method, uri);

        let mut req = Request::new(Body::empty());
        *req.method_mut() = method;
        *req.uri_mut() = uri;

        req.headers_mut()
            .insert("X-Riot-Token", HeaderValue::from_str(&self.api_key)?);

        debug!("{:?}", req);

        Ok(req)
    }

    fn get_uri(&self, path: String) -> Uri {
        format!("https://{}{}", self.platform.host(&self.api_host), path)
            .parse::<Uri>()
            .unwrap()
    }
}

impl<T> Clone for Api<T> where T: Clone {
    fn clone(&self) -> Self {
        Self {
            api_host: self.api_host.clone(),
            api_key: self.api_key.clone(),
            platform: self.platform.clone(),
            client: self.client.clone(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type FutureResult<T> = dyn Future<Item = T, Error = Error>;
pub type HttpsClient = Client<HttpsConnector<HttpConnector>>;
