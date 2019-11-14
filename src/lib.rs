#[macro_use]
extern crate failure;

mod api;
mod hosts;
pub mod models;

pub use self::api::*;
pub use self::hosts::*;

use crate::hosts::WithHosts;
use failure::Error;
use hyper::client::HttpConnector;
use hyper::header::HeaderValue;
use hyper::rt::{Future, Stream};
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use log::debug;
use serde::Deserialize;
use std::fmt::Debug;

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

    pub fn champion(&self) -> ChampionApi<T> {
        let api = self.clone();
        ChampionApi::new(api)
    }

    pub fn champion_mastery(&self) -> ChampionMasteryApi<T> {
        let api = self.clone();
        ChampionMasteryApi::new(api)
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

    fn client_request<'a, V: Debug>(&self, path: String) -> impl Future<Item = V, Error = Error>
    where
        for<'de> V: Deserialize<'de> + 'a,
    {
        let req = self.build_request(Method::GET, path).unwrap();

        self.client
            .request(req)
            .and_then(|res| res.into_body().concat2())
            .map_err(Error::from)
            .and_then(|chunk| {
                let data = serde_json::from_slice(&chunk)?;
                debug!("{:?}", data);

                Ok(data)
            })
            .from_err()
    }
}

impl<T> Clone for Api<T>
where
    T: Clone,
{
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
