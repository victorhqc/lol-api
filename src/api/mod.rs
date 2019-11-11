mod summoner;
pub use self::summoner::*;

use hyper::{Client, Uri};
use hyper::rt::Future;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use failure::Error;
use crate::regions::{WithHosts, Platforms};

pub struct Api {
    api_key: String,
    client: HttpsClient,
    platform: Platforms,
}

impl Api {
    pub fn new(api_key: String, platform: Platforms) -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

        Self {
            api_key,
            platform,
            client,
        }
    }

    pub fn get_url(&self, path: String) -> Uri {
        format!("{}{}", self.platform.host(), path).parse::<Uri>().unwrap()
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type FutureResult<T> = dyn Future<Item=T, Error=Error>;
pub type HttpsClient = Client<HttpsConnector<HttpConnector>>;
