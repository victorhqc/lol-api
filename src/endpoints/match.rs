use failure::Error;
use hyper::rt::Future;

use crate::{
    constants::{Division, Queue, Tier, WithHost},
    models::{LeagueEntryDTO, LeagueListDTO},
    RiotApi,
};

use super::MATCH_V4_PATH;

/// MatchV4 endpoints
///
/// <a href="https://developer.riotgames.com/apis#match-v4">Official API Documentation</a>
pub struct MatchV4<'a> {
    pub api: &'a RiotApi,
}

impl<'a> MatchV4<'a> {
    pub fn new(api: &'a RiotApi) -> Self {
        Self { api }
    }

    /// Get match IDs by tournament code.
    ///
    /// <a href="https://developer.riotgames.com/apis#match-v4/GET_getMatchIdsByTournamentCode">
    ///   Official API Documentation
    /// </a>
    pub fn get_match_ids_by_tournament_code<T: WithHost>(
        &self,
        region: T,
        tournament_code: &str,
    ) -> impl Future<Item = Vec<String>, Error = Error> {
        let path = format!(
            "{}/by-tournament-code/{}/ids",
            MATCH_V4_PATH, tournament_code
        );

        self.api.get(region, path)
    }


}
