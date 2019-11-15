use failure::Error;
use hyper::rt::Future;

use crate::{RiotApi, WithHosts};
use crate::models::Summoner;

use super::SUMMONER_API_PATH;

pub struct SummonerApi<'a, T> {
    api: &'a RiotApi<T>,
}

impl<'a, T> SummonerApi<'a, T>
where
    T: WithHosts,
{
    pub fn new(api: &'a RiotApi<T>) -> Self {
        Self { api }
    }

    pub fn by_name(&self, name: &str) -> impl Future<Item = Summoner, Error = Error> {
        let path = format!("{}/by-name/{}", SUMMONER_API_PATH, name);
        self.api.get(path)
    }

    pub fn by_account_id(&self, account_id: &str) -> impl Future<Item = Summoner, Error = Error> {
        let path = format!("{}/by-account/{}", SUMMONER_API_PATH, account_id);
        self.api.get(path)
    }

    pub fn by_puuid(&self, puuid: &str) -> impl Future<Item = Summoner, Error = Error> {
        let path = format!("{}/by-puuid/{}", SUMMONER_API_PATH, puuid);
        self.api.get(path)
    }

    pub fn by_summoner_id(&self, id: &str) -> impl Future<Item = Summoner, Error = Error> {
        let path = format!("{}/{}", SUMMONER_API_PATH, id);
        self.api.get(path)
    }
}
