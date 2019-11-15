use failure::Error;
use hyper::rt::Future;

use crate::{constants::WithHost, models::ChampionInfo, RiotApi};

use super::CHAMPION_ROTATIONS_PATH;

/// ChampionV3 endpoints.
///
/// <a href="https://developer.riotgames.com/apis#champion-v3">Official API Documentation</a>
pub struct ChampionV3<'a> {
    pub api: &'a RiotApi,
}

impl<'a> ChampionV3<'a> {
    pub fn new(api: &'a RiotApi) -> Self {
        Self { api }
    }

    /// Returns champion rotations, including free-to-play and low-level free-to-play rotations (REST)
    ///
    /// <a href="https://developer.riotgames.com/apis#champion-v3/GET_getChampionInfo">Official API Documentation</a>
    /// # Parameters
    /// * `region` - Region to query.
    pub fn get_champion_info<T: WithHost>(
        &self,
        region: T,
    ) -> impl Future<Item = ChampionInfo, Error = Error> {
        self.api.get(region, CHAMPION_ROTATIONS_PATH.to_string())
    }
}
