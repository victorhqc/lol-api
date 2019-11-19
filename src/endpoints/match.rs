use failure::Error;
use hyper::rt::Future;

use crate::{
    constants::{WithHost, Queue},
    models::{MatchDTO, MatchlistDTO},
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
    ) -> impl Future<Item = Vec<u64>, Error = Error> {
        let path = format!(
            "{}/by-tournament-code/{}/ids",
            MATCH_V4_PATH, tournament_code
        );

        self.api.get(region, path)
    }

    /// Get match by match ID.
    ///
    /// <a href="https://developer.riotgames.com/apis#match-v4/GET_getMatch">
    ///   Official API Documentation
    /// </a>
    pub fn get_match<T: WithHost>(
        &self,
        region: T,
        match_id: &str,
    ) -> impl Future<Item = MatchDTO, Error = Error> {
        let path = format!(
            "{}/matches/{}",
            MATCH_V4_PATH, match_id,
        );

        self.api.get(region, path)
    }

    /// Get match by match ID and tournament Code.
    ///
    /// <a href="https://developer.riotgames.com/apis#match-v4/GET_getMatchByTournamentCode">
    ///   Official API Documentation
    /// </a>
    pub fn get_match_by_tournament_code<T: WithHost>(
        &self,
        region: T,
        match_id: &str,
        tournament_code: &str,
    ) -> impl Future<Item = MatchDTO, Error = Error> {
        let path = format!(
            "{}/matches/{}/by-tournament-code/{}",
            MATCH_V4_PATH, match_id, tournament_code
        );

        self.api.get(region, path)
    }

    /// Get matchlist for games played given account ID and platform ID and filtered using given
    /// fulter parameters, if any.
    ///
    /// <a href="https://developer.riotgames.com/apis#match-v4/GET_getMatchlist">
    ///   Official API Documentation
    /// </a>
    pub fn get_matchlist<T: WithHost>(
        &self,
        region: T,
        encrypted_account_id: &str,
        parameters: GetMatchlistParameters,
    ) -> impl Future<Item = MatchlistDTO, Error = Error> {
        let path = format!(
            "{}/matchlists/by-account/{}",
            MATCH_V4_PATH, encrypted_account_id
        );

        let mut parsed_parameters: Vec<(String, String)> = Vec::new();
        match parameters.champion {
            Some(champion) => {
                let parsed_champions: String = champion
                    .iter()
                    .map(|c| Into::<i32>::into(*c).to_string())
                    .collect();

                parsed_parameters.push((String::from("champion"), parsed_champions));
            }
            None => {}
        };

        self.api.get_with_params(region, path, parsed_parameters)
    }
}

pub struct GetMatchlistParameters {
    pub champion: Option<Vec<i32>>, // TODO: Implement Champion constant
    pub queue: Option<Vec<Queue>>,
    pub end_time: Option<u64>,
    pub begin_time: Option<u64>,
    pub end_index: Option<u64>,
    pub begin_index: Option<u64>,
}
