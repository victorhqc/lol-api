use crate::regions::WithHosts;
use chrono::prelude::*;
use failure::Error;
use hyper::rt::{Future, Stream};
use hyper::{Body, Method, Request};
use serde_derive::{Deserialize, Serialize};
use log::{debug};

use crate::api::Api;

pub struct SummonerApi<T> {
    api: Api<T>,
}

impl<T> SummonerApi<T>
where
    T: WithHosts,
{
    pub fn new(api_key: String, platform: T, api_host: Option<String>) -> Self {
        let api = Api::new(api_key, platform, api_host);

        Self { api }
    }

    pub fn by_name(&self, name: &String) -> impl Future<Item = Summoner, Error = Error> {
        let path = get_summoner_path("/by-name/", name);
        let req = self.api.build_request(Method::GET, path);
        self.get_summoner(req.unwrap())
    }

    pub fn get_summoner(&self, req: Request<Body>) -> impl Future<Item = Summoner, Error = Error> {
        self.api
            .client
            .request(req)
            .and_then(|res| res.into_body().concat2())
            .from_err()
            .and_then(|body| {
                debug!("{:?}", body);

                let summoner = serde_json::from_slice(&body)?;

                debug!("SUMMONER! {:?}", summoner);

                Ok(summoner)
            })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    id: String,
    account_id: String,
    puuid: String,
    name: String,
    profile_icon_id: u16,
    revision_date: u32,
    summoner_level: NaiveDateTime,
}

pub const SUMMONER_API_PATH: &'static str = "/lol/summoner/v4/summoners";
fn get_summoner_path(route: &str, param: &String) -> String {
    String::from(format!("{}{}{}", SUMMONER_API_PATH, route, param))
}
