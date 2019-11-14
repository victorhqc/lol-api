use failure::Error;
use hyper::rt::Future;

use crate::Api;
use crate::hosts::WithHosts;
use crate::models::Summoner;

use super::SUMMONER_API_PATH;

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

fn get_summoner_path(route: &str, param: &str) -> String {
    String::from(format!("{}{}{}", SUMMONER_API_PATH, route, param))
}
