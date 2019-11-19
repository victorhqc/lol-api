use failure::Error;
use hyper::rt::Future;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use url::form_urlencoded::Serializer;

use crate::{
    constants::{Division, Queue, Tier, WithHost},
    models::{LeagueEntryDTO, LeagueListDTO},
    RiotApi,
};

use super::LEAGUE_PATH;

/// LeagueV4 endpoints.
///
/// <a href="https://developer.riotgames.com/apis#league-v4">Official API Documentation</a>
pub struct LeagueV4<'a> {
    pub api: &'a RiotApi,
}

impl<'a> LeagueV4<'a> {
    pub fn new(api: &'a RiotApi) -> Self {
        Self { api }
    }

    /// Get the challenger league for given queue.
    ///
    /// <a href="https://developer.riotgames.com/apis#league-v4/GET_getChallengerLeague">
    ///   Official API Documentation
    /// </a>
    pub fn get_challenger_league<T: WithHost>(
        &self,
        region: T,
        queue: Queue,
    ) -> impl Future<Item = LeagueListDTO, Error = Error> {
        let path = format!("{}/challengerleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.get(region, path)
    }

    /// Get the grandmaster league of a specific queue.
    ///
    /// <a href="https://developer.riotgames.com/apis#league-v4/GET_getGrandmasterLeague">
    ///   Official API Documentation
    /// </a>
    pub fn get_grandmaster_league<T: WithHost>(
        &self,
        region: T,
        queue: Queue,
    ) -> impl Future<Item = LeagueListDTO, Error = Error> {
        let path = format!("{}/grandmasterleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.get(region, path)
    }

    /// Get the master league for given queue.
    ///
    /// <a href="https://developer.riotgames.com/apis#league-v4/GET_getMasterLeague">
    ///   Official API Documentation
    /// </a>
    pub fn get_master_league<T: WithHost>(
        &self,
        region: T,
        queue: Queue,
    ) -> impl Future<Item = LeagueListDTO, Error = Error> {
        let path = format!("{}/masterleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.get(region, path)
    }

    /// Get league with given ID, including inactive entries.
    ///
    /// <a href="https://developer.riotgames.com/apis#league-v4/GET_getLeagueById">
    ///   Official API Documentation
    ///</a>
    pub fn get_league_by_id<T: WithHost>(
        &self,
        region: T,
        league_id: &str,
    ) -> impl Future<Item = LeagueListDTO, Error = Error> {
        let path = format!("{}/leagues/{}", LEAGUE_PATH, league_id);

        self.api.get(region, path)
    }

    /// Get league entries in all queues for a given summoner ID.
    ///
    /// <a href="https://developer.riotgames.com/apis#league-v4/GET_getLeagueEntriesForSummoner">
    ///   Official API Documentation
    /// </a>
    pub fn get_league_entries_for_summoner<T: WithHost>(
        &self,
        region: T,
        summoner_id: &str,
    ) -> impl Future<Item = Vec<LeagueEntryDTO>, Error = Error> {
        let path = format!("{}/entries/by-summoner/{}", LEAGUE_PATH, summoner_id);

        self.api.get(region, path)
    }

    /// Get all the league entries.
    ///
    /// <a href="https://developer.riotgames.com/apis#league-v4/GET_getLeagueEntries">
    ///   Official API Documentation
    ///</a>
    pub fn get_league_entries<T: WithHost>(
        &self,
        region: T,
        queue: Queue,
        tier: Tier,
        division: Division,
        parameters: GetLeagueEntriesParams,
    ) -> impl Future<Item = Vec<LeagueEntryDTO>, Error = Error> {
        let path = format!("{}/entries/{}/{}/{}", LEAGUE_PATH, queue, tier, division);

        let mut query_params = Serializer::new(String::new());
        for parameter in LeagueParams::iter() {
            match parameter {
                LeagueParams::Page => match &parameters.page {
                    Some(page) => {
                        query_params.append_pair("page", &page.to_string());
                    }
                    None => {}
                }
            }
        }

        self.api.get_with_params(region, path, query_params.finish())
    }
}

pub struct GetLeagueEntriesParams {
    page: Option<u32>,
}

#[derive(EnumIter, Debug)]
enum LeagueParams {
    Page,
}
