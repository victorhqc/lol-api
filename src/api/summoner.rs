use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use failure::Error;
use hyper::rt::Future;
use serde_derive::{Deserialize, Serialize};

use crate::api::Api;
use crate::regions::WithHosts;

pub struct SummonerApi<'a, T> {
    api: &'a Api<T>,
}

impl<'a, T> SummonerApi<'a, T>
where
    T: WithHosts,
{
    pub fn new(api: &'a Api<T>) -> Self {
        Self { api }
    }

    pub fn by_name(&self, name: &str) -> impl Future<Item = Summoner, Error = Error> {
        let path = get_summoner_path("/by-name/", name);
        self.api.client_request::<Summoner>(path)
    }

    pub fn by_account_id(&self, account_id: &str) -> impl Future<Item = Summoner, Error = Error> {
        let path = get_summoner_path("/by-account/", account_id);
        self.api.client_request::<Summoner>(path)
    }

    pub fn by_puuid(&self, puuid: &str) -> impl Future<Item = Summoner, Error = Error> {
        let path = get_summoner_path("/by-puuid/", puuid);
        self.api.client_request::<Summoner>(path)
    }

    pub fn by_summoner_id(&self, id: &str) -> impl Future<Item = Summoner, Error = Error> {
        let path = get_summoner_path("/", id);
        self.api.client_request::<Summoner>(path)
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
