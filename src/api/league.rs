use failure::Error;
use hyper::rt::Future;

use crate::models::{LeagueEntry, LeagueList, Queue, Rank, Tier};
use crate::{Api, WithHosts};

use super::LEAGUE_PATH;

pub struct LeagueApi<'a, T> {
    pub api: &'a Api<T>,
}

impl<'a, T> LeagueApi<'a, T>
where
    T: WithHosts,
{
    pub fn new(api: &'a Api<T>) -> Self {
        Self { api }
    }

    pub fn challenger_leagues_by_queue(
        &self,
        queue: Queue,
    ) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/challengerleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.client_request(path)
    }

    pub fn grandmaster_leagues_by_queue(
        &self,
        queue: Queue,
    ) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/grandmasterleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.client_request(path)
    }

    pub fn master_leagues_by_queue(
        &self,
        queue: Queue,
    ) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/masterleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.client_request(path)
    }

    pub fn leagues_by_id(&self, league_id: &str) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/leagues/{}", LEAGUE_PATH, league_id);

        self.api.client_request(path)
    }

    pub fn entries_by_summoner_id(
        &self,
        summoner_id: &str,
    ) -> impl Future<Item = Vec<LeagueEntry>, Error = Error> {
        let path = format!("{}/entries/by-summoner/{}", LEAGUE_PATH, summoner_id);

        self.api.client_request(path)
    }

    pub fn entries(
        &self,
        queue: Queue,
        tier: Tier,
        rank: Rank,
    ) -> impl Future<Item = Vec<LeagueEntry>, Error = Error> {
        let path = format!("{}/entries/{}/{}/{}", LEAGUE_PATH, queue, tier, rank);

        self.api.client_request(path)
    }
}
