use failure::Error;
use futures::future;
use hyper::client::HttpConnector;
use hyper::header::HeaderValue;
use hyper::rt::{Future, Stream};
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use log::debug;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::constants::WithHost;
use crate::endpoints::{ChampionMasteryV4, ChampionV3, LeagueV4, MatchV4, SummonerV4};
use crate::FetchError;

pub struct RiotApi {
    config: RustApiConfig,
}

impl RiotApi {
    pub fn new(config: RustApiConfig) -> Self {
        Self { config }
    }

    /// Handle for SummonerV4
    ///
    /// <a href="https://developer.riotgames.com/apis#summoner-v4">Official API Documentation</a>
    pub fn summoner_v4(&self) -> SummonerV4 {
        SummonerV4::new(self)
    }

    /// Handle for ChampionV3 endpoints.
    ///
    /// <a href="https://developer.riotgames.com/apis#champion-v3">Official API Documentation</a>
    pub fn champion_v3(&self) -> ChampionV3 {
        ChampionV3::new(self)
    }

    /// Handle for LeagueV4 endpoints.
    ///
    /// <a href="https://developer.riotgames.com/apis#league-v4">Official API Documentation</a>
    pub fn league_v4(&self) -> LeagueV4 {
        LeagueV4::new(self)
    }

    /// Handle for ChampionMasteryV4 endpoints.
    ///
    /// <a href="https://developer.riotgames.com/apis#champion-mastery-v4">Official API Documentation</a>
    pub fn champion_mastery_v4(&self) -> ChampionMasteryV4 {
        ChampionMasteryV4::new(self)
    }

    /// Handle for MatchV4 endpoints.
    ///
    /// <a href="https://developer.riotgames.com/apis#match-v4">Official API Documentation</a>
    pub fn match_v4(&self) -> MatchV4 {
        MatchV4::new(self)
    }

    pub fn build_request<T: WithHost>(
        &self,
        method: Method,
        region: T,
        path: String,
        params: String,
    ) -> Result<Request<Body>> {
        let uri = self.forge_uri(region, path, params)?;
        debug!("{}: {}", method, uri);

        let mut req = Request::new(Body::empty());
        *req.method_mut() = method;
        *req.uri_mut() = uri;

        req.headers_mut()
            .insert("X-Riot-Token", HeaderValue::from_str(&self.config.api_key)?);

        debug!("{:?}", req);

        Ok(req)
    }

    fn forge_uri<T: WithHost>(
        &self,
        region: T,
        path: String,
        params: String,
    ) -> std::result::Result<Uri, Error> {
        let uri = format!(
            "https://{}{}?{}",
            region.host(&self.config.api_host),
            path,
            params
        )
        .parse::<Uri>()?;

        Ok(uri)
    }

    pub fn get_with_params<'a, R, T>(
        &self,
        region: T,
        path: String,
        params: String,
    ) -> impl Future<Item = R, Error = Error>
    where
        R: DeserializeOwned + Debug,
        T: WithHost,
    {
        self.get_data(region, path, params)
    }

    pub fn get<'a, R, T>(&self, region: T, path: String) -> impl Future<Item = R, Error = Error>
    where
        R: DeserializeOwned + Debug,
        T: WithHost,
    {
        self.get_data(region, path, String::from(""))
    }

    fn get_data<'a, R, T>(
        &self,
        region: T,
        path: String,
        params: String,
    ) -> impl Future<Item = R, Error = Error>
    where
        R: DeserializeOwned + Debug,
        T: WithHost,
    {
        let req = self
            .build_request(Method::GET, region, path, params)
            .unwrap();

        self.config
            .client
            .request(req)
            .from_err::<FetchError>()
            .and_then(|res| {
                if !res.status().is_success() {
                    future::Either::A(future::err(FetchError::Status(res.status())))
                } else {
                    future::Either::B(res.into_body().concat2().from_err())
                }
            })
            .from_err::<FetchError>()
            .and_then(|chunk| {
                let data = serde_json::from_slice(&chunk)?;
                debug!("{:?}", data);

                Ok(data)
            })
            .from_err()
    }
}

pub struct RustApiConfig {
    api_host: String,
    api_key: String,
    client: HttpsClient,
}

impl RustApiConfig {
    pub fn new(api_key: String, api_host: Option<String>) -> Self {
        let api_host = match api_host {
            Some(h) => h,
            None => String::from("api.riotgames.com"),
        };

        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, hyper::Body>(https);

        Self {
            api_host,
            api_key,
            client,
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
