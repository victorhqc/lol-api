use failure::Error;
use hyper::rt::Future;

use crate::{constants::WithHost, models::Summoner, RiotApi};

use super::SUMMONER_API_PATH;

pub struct SummonerV4<'a> {
    api: &'a RiotApi,
}

impl<'a> SummonerV4<'a> {
    pub fn new(api: &'a RiotApi) -> Self {
        Self { api }
    }

    pub fn by_name<T: WithHost>(
        &self,
        region: T,
        name: &str,
    ) -> impl Future<Item = Summoner, Error = Error> {
        let path = format!("{}/by-name/{}", SUMMONER_API_PATH, name);
        self.api.get(region, path)
    }

    pub fn by_account_id<T: WithHost>(
        &self,
        region: T,
        account_id: &str,
    ) -> impl Future<Item = Summoner, Error = Error> {
        let path = format!("{}/by-account/{}", SUMMONER_API_PATH, account_id);
        self.api.get(region, path)
    }

    pub fn by_puuid<T: WithHost>(
        &self,
        region: T,
        puuid: &str,
    ) -> impl Future<Item = Summoner, Error = Error> {
        let path = format!("{}/by-puuid/{}", SUMMONER_API_PATH, puuid);
        self.api.get(region, path)
    }

    pub fn by_summoner_id<T: WithHost>(
        &self,
        region: T,
        id: &str,
    ) -> impl Future<Item = Summoner, Error = Error> {
        let path = format!("{}/{}", SUMMONER_API_PATH, id);
        self.api.get(region, path)
    }
}
