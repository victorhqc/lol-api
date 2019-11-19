use failure::Error;
use futures::future;
use hyper::client::HttpConnector;
use hyper::rt::{Future, Stream};
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use log::debug;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::endpoints::Constants;
use crate::FetchError;

pub struct FetchApi {
    config: FetchApiConfig,
}

impl FetchApi {
    pub fn new(config: FetchApiConfig) -> Self {
        Self { config }
    }

    pub fn constants(&self) -> Constants {
        Constants::new(self)
    }

    pub fn build_request(
        &self,
        method: Method,
        path: String,
        params: String,
    ) -> FetchResult<Request<Body>> {
        let uri = self.forge_uri(path, params)?;
        debug!("{}: {}", method, uri);

        let mut req = Request::new(Body::empty());
        *req.method_mut() = method;
        *req.uri_mut() = uri;

        debug!("{:?}", req);

        Ok(req)
    }

    fn forge_uri(
        &self,
        path: String,
        params: String,
    ) -> std::result::Result<Uri, Error> {
        let uri = format!(
            "https://{}/docs/lol/{}?{}",
            self.config.api_host,
            path,
            params
        )
        .parse::<Uri>()?;

        Ok(uri)
    }

    pub fn get_with_params<R>(
        &self,
        path: String,
        params: String,
    ) -> impl Future<Item = R, Error = Error>
    where
        R: DeserializeOwned + Debug,
    {
        self.get_data(path, params)
    }

    pub fn get<'a, R>(&self, path: String) -> impl Future<Item = R, Error = Error>
    where
        R: DeserializeOwned + Debug
    {
        self.get_data(path, String::from(""))
    }

    fn get_data<'a, R>(
        &self,
        path: String,
        params: String,
    ) -> impl Future<Item = R, Error = Error>
    where
        R: DeserializeOwned + Debug,
    {
        let req = self
            .build_request(Method::GET, path, params)
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

pub struct FetchApiConfig {
    api_host: String,
    client: HttpsClient,
}

impl FetchApiConfig {
    pub fn new(api_host: Option<String>) -> Self {
        let api_host = match api_host {
            Some(h) => h,
            None => String::from("static.developer.riotgames.com"),
        };

        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, hyper::Body>(https);

        Self {
            api_host,
            client,
        }
    }
}

type FetchResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type FutureResult<T> = dyn Future<Item = T, Error = Error>;
pub type HttpsClient = Client<HttpsConnector<HttpConnector>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
