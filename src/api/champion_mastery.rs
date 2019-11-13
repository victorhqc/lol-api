use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use failure::Error;
use hyper::rt::Future;
use serde_derive::Deserialize;

use crate::api::Api;
use crate::regions::WithHosts;

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
    pub chest_granted: bool,
    pub champion_level: u8,
    pub champion_id: u32,
    pub champion_points_until_next_level: u32,
    #[serde(with = "ts_milliseconds")]
    pub last_play_time: DateTime<Utc>,
    pub champion_points_since_last_level: u32,
    summoner_id: String,
}

pub const CHAMPION_MASTERY_PATH: &'static str = "/lol/champion-mastery/v4/champion-masteries";
pub const SCORE_MASTERY_PATH: &'static str = "/lol/champion-mastery/v4/scores";

fn get_champion_mastery_path(route: &str, param: &str) -> String {
    String::from(format!("{}{}{}", CHAMPION_MASTERY_PATH, route, param))
}
