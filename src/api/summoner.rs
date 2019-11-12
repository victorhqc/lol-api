use failure::Error;
use hyper::rt::{Future, Stream};
use hyper::{Method};
use serde_derive::{Deserialize, Serialize};
use log::{debug};
use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds;

use crate::api::Api;
use crate::regions::WithHosts;

pub struct SummonerApi {}

impl SummonerApi {
    pub fn new() -> Self {
        Self { }
    }

    pub fn by_name<T: WithHosts>(&self, api: &Api<T>, name: &str) -> impl Future<Item = Summoner, Error = Error> {
        debug!("Summoner by name: {}", name);
        let path = get_summoner_path("/by-name/", name);
        self.get_summoner(api, path)
    }

    pub fn by_account_id<T: WithHosts>(&self, api: &Api<T>, account_id: &str) -> impl Future<Item = Summoner, Error = Error> {
        debug!("Summoner by account_id: {}", account_id);
        let path = get_summoner_path("/by-account/", account_id);
        self.get_summoner(api, path)
    }

    pub fn by_puuid<T: WithHosts>(&self, api: &Api<T>, puuid: &str) -> impl Future<Item = Summoner, Error = Error> {
        debug!("Summoner by puuid: {}", puuid);
        let path = get_summoner_path("/by-puuid/", puuid);
        self.get_summoner(api, path)
    }

    pub fn by_summoner_id<T: WithHosts>(&self, api: &Api<T>, id: &str) -> impl Future<Item = Summoner, Error = Error> {
        debug!("Summoner by id: {}", id);
        let path = get_summoner_path("/", id);
        self.get_summoner(api, path)
    }

    fn get_summoner<T: WithHosts>(&self, api: &Api<T>, path: String) -> impl Future<Item = Summoner, Error = Error> {
        let req = api.build_request(Method::GET, path).unwrap();

        api
            .client
            .request(req)
            .and_then(|res| res.into_body().concat2())
            .from_err()
            .and_then(|body| {
                let summoner = serde_json::from_slice(&body)?;

                debug!("{:?}", summoner);

                Ok(summoner)
            })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub id: String,
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    pub profile_icon_id: u16,
    pub summoner_level: u16,
    #[serde(with = "ts_milliseconds")]
    pub revision_date: DateTime<Utc>,
}

pub const SUMMONER_API_PATH: &'static str = "/lol/summoner/v4/summoners";
fn get_summoner_path(route: &str, param: &str) -> String {
    String::from(format!("{}{}{}", SUMMONER_API_PATH, route, param))
}
