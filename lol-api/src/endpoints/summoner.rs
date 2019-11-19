use failure::Error;
use hyper::rt::Future;

use crate::{constants::WithHost, models::SummonerDTO, RiotApi};

use super::SUMMONER_API_PATH;

/// SummonerV4 endpoints.
///
/// <a href="https://developer.riotgames.com/apis#summoner-v4">Official Documentation</a>
pub struct SummonerV4<'a> {
    api: &'a RiotApi,
}

impl<'a> SummonerV4<'a> {
    pub fn new(api: &'a RiotApi) -> Self {
        Self { api }
    }

    /// Get a summoner by summoner name.
    ///
    /// <a href="https://developer.riotgames.com/apis#summoner-v4/GET_getBySummonerName">
    ///   Official API Documentation
    /// </a>
    pub fn get_by_summoner_name<T: WithHost>(
        &self,
        region: T,
        summoner_name: &str,
    ) -> impl Future<Item = SummonerDTO, Error = Error> {
        let path = format!("{}/by-name/{}", SUMMONER_API_PATH, summoner_name);
        self.api.get(region, path)
    }

    /// Get a summoner by account ID.
    ///
    /// <a href="https://developer.riotgames.com/apis#summoner-v4/GET_getByAccountId">
    ///   Official API Documentation
    /// </a>
    pub fn get_by_account_id<T: WithHost>(
        &self,
        region: T,
        encrypted_account_id: &str,
    ) -> impl Future<Item = SummonerDTO, Error = Error> {
        let path = format!("{}/by-account/{}", SUMMONER_API_PATH, encrypted_account_id);
        self.api.get(region, path)
    }

    /// Get a summoner by PUUID.
    ///
    /// <a href="https://developer.riotgames.com/apis#summoner-v4/GET_getByPUUID">
    ///   Official API Documentation
    /// </a>
    pub fn get_by_puuid<T: WithHost>(
        &self,
        region: T,
        encrypted_puuid: &str,
    ) -> impl Future<Item = SummonerDTO, Error = Error> {
        let path = format!("{}/by-puuid/{}", SUMMONER_API_PATH, encrypted_puuid);
        self.api.get(region, path)
    }

    /// Get a summoner by summoner ID.
    /// <a href="https://developer.riotgames.com/apis#summoner-v4/GET_getBySummonerId">
    ///   Official API Documentation
    /// </a>
    pub fn get_by_summoner_id<T: WithHost>(
        &self,
        region: T,
        encrypted_summoner_id: &str,
    ) -> impl Future<Item = SummonerDTO, Error = Error> {
        let path = format!("{}/{}", SUMMONER_API_PATH, encrypted_summoner_id);
        self.api.get(region, path)
    }
}
