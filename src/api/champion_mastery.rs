use failure::Error;
use hyper::rt::Future;

use crate::Api;
use crate::hosts::WithHosts;
use crate::models::ChampionMastery;

use super::{CHAMPION_MASTERY_PATH, SCORE_MASTERY_PATH};

pub struct ChampionMasteryApi<'a, T> {
    api: &'a Api<T>,
}

impl<'a, T> ChampionMasteryApi<'a, T>
where
    T: WithHosts,
{
    pub fn new(api: &'a Api<T>) -> Self {
        Self { api }
    }

    pub fn by_summoner_id(
        &self,
        summoner_id: &str,
    ) -> impl Future<Item = Vec<ChampionMastery>, Error = Error> {
        let path = get_champion_mastery_path("/by-summoner/", summoner_id);
        self.api.client_request::<Vec<ChampionMastery>>(path)
    }

    pub fn by_champion_id(
        &self,
        summoner_id: &str,
        champion_id: u32,
    ) -> impl Future<Item = ChampionMastery, Error = Error> {
        let path = String::from(format!(
            "{}{}{}{}{}",
            CHAMPION_MASTERY_PATH, "/by-summoner/", summoner_id, "/by-champion/", champion_id
        ));
        self.api.client_request::<ChampionMastery>(path)
    }

    pub fn total_score(&self, summoner_id: &str) -> impl Future<Item = u32, Error = Error> {
        let path = String::from(format!(
            "{}{}{}",
            SCORE_MASTERY_PATH, "/by-summoner/", summoner_id,
        ));

        self.api.client_request::<u32>(path)
    }
}

fn get_champion_mastery_path(route: &str, param: &str) -> String {
    String::from(format!("{}{}{}", CHAMPION_MASTERY_PATH, route, param))
}
