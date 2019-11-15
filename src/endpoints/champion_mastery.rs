use failure::Error;
use hyper::rt::Future;

use super::{CHAMPION_MASTERY_PATH, SCORE_MASTERY_PATH};
use crate::{constants::WithHost, models::ChampionMasteryDTO, RiotApi};

/// ChampionMasteryV4 endpoints.
///
/// <a href="https://developer.riotgames.com/apis#champion-mastery-v4">Official API Documentation</a>
pub struct ChampionMasteryV4<'a> {
    api: &'a RiotApi,
}

impl<'a> ChampionMasteryV4<'a> {
    pub fn new(api: &'a RiotApi) -> Self {
        Self { api }
    }

    /// Get all champion mastery entries sorted by number of champion points descending
    ///
    /// <a href="https://developer.riotgames.com/api-methods/#champion-mastery-v4/GET_getAllChampionMasteries">Official API Documentation</a>
    /// # Parameters
    /// * `region` - Region to query.
    /// * `encrypted_summoner_id` - Summoner ID associated with the player
    pub fn get_all_champion_masteries<T: WithHost>(
        &self,
        region: T,
        encrypted_summoner_id: &str,
    ) -> impl Future<Item = Vec<ChampionMasteryDTO>, Error = Error> {
        let path = format!("{}/by-summoner/{}", CHAMPION_MASTERY_PATH, encrypted_summoner_id);
        self.api.get(region, path)
    }

    /// Get a champion mastery by player ID and champion ID
    ///
    /// <a href="https://developer.riotgames.com/apis#champion-mastery-v4/GET_getChampionMastery">Official API Documentation</a>
    /// # Parameters
    /// * `region` - Region to execute against
    /// * `encrypted_summoner_id` - Summoner ID associated with the player
    /// * `champion_id` - Champion ID for fetching mastery
    pub fn get_champion_mastery<T: WithHost>(
        &self,
        region: T,
        encrypted_summoner_id: &str,
        champion_id: u32,
    ) -> impl Future<Item = ChampionMasteryDTO, Error = Error> {
        let path = format!(
            "{}/by-summoner/{}/by-champion/{}",
            CHAMPION_MASTERY_PATH, encrypted_summoner_id, champion_id,
        );
        self.api.get(region, path)
    }

    /// Get a player's total champion mastery score, which is the sum of individual champion mastery levels
    ///
    /// <a href="https://developer.riotgames.com/apis#champion-mastery-v4/GET_getChampionMasteryScore">Official API Documentation</a>
    /// # Parameters
    /// * `region` - Region to execute against
    /// * `encrypted_summoner_id` - Summoner ID associated with the player
    pub fn get_champion_mastery_score<T: WithHost>(
        &self,
        region: T,
        encrypted_summoner_id: &str,
    ) -> impl Future<Item = u32, Error = Error> {
        let path = format!("{}/by-summoner/{}", SCORE_MASTERY_PATH, encrypted_summoner_id,);

        self.api.get(region, path)
    }
}
