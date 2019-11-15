use failure::Error;
use hyper::rt::Future;

use super::{CHAMPION_MASTERY_PATH, SCORE_MASTERY_PATH};
use crate::{constants::WithHost, models::ChampionMastery, RiotApi};

pub struct ChampionMasteryV4<'a> {
    api: &'a RiotApi,
}

impl<'a> ChampionMasteryV4<'a> {
    pub fn new(api: &'a RiotApi) -> Self {
        Self { api }
    }

    pub fn by_summoner_id<T: WithHost>(
        &self,
        platform: T,
        summoner_id: &str,
    ) -> impl Future<Item = Vec<ChampionMastery>, Error = Error> {
        let path = format!("{}/by-summoner/{}", CHAMPION_MASTERY_PATH, summoner_id);
        self.api.get(platform, path)
    }

    pub fn by_summoners_champion<T: WithHost>(
        &self,
        platform: T,
        summoner_id: &str,
        champion_id: u32,
    ) -> impl Future<Item = ChampionMastery, Error = Error> {
        let path = format!(
            "{}/by-summoner/{}/by-champion/{}",
            CHAMPION_MASTERY_PATH, summoner_id, champion_id,
        );
        self.api.get(platform, path)
    }

    pub fn total_score<T: WithHost>(
        &self,
        platform: T,
        summoner_id: &str,
    ) -> impl Future<Item = u32, Error = Error> {
        let path = format!("{}/by-summoner/{}", SCORE_MASTERY_PATH, summoner_id,);

        self.api.get(platform, path)
    }
}
