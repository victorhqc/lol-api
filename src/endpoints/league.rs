use failure::Error;
use hyper::rt::Future;

use crate::models::{LeagueEntry, LeagueList, Queue, Rank, Tier};
use crate::{RiotApi, WithHosts};

use super::LEAGUE_PATH;

pub struct LeagueV4<'a, T> {
    pub api: &'a RiotApi<T>,
}

impl<'a, T> LeagueV4<'a, T>
where
    T: WithHosts,
{
    pub fn new(api: &'a RiotApi<T>) -> Self {
        Self { api }
    }

    pub fn challenger_leagues_by_queue(
        &self,
        queue: Queue,
    ) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/challengerleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.get(path)
    }

    pub fn grandmaster_leagues_by_queue(
        &self,
        queue: Queue,
    ) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/grandmasterleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.get(path)
    }

    pub fn master_leagues_by_queue(
        &self,
        queue: Queue,
    ) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/masterleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.get(path)
    }

    pub fn leagues_by_id(&self, league_id: &str) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/leagues/{}", LEAGUE_PATH, league_id);

        self.api.get(path)
    }

    pub fn entries_by_summoner_id(
        &self,
        summoner_id: &str,
    ) -> impl Future<Item = Vec<LeagueEntry>, Error = Error> {
        let path = format!("{}/entries/by-summoner/{}", LEAGUE_PATH, summoner_id);

        self.api.get(path)
    }

    pub fn entries(
        &self,
        queue: Queue,
        tier: Tier,
        rank: Rank,
    ) -> impl Future<Item = Vec<LeagueEntry>, Error = Error> {
        let path = format!("{}/entries/{}/{}/{}", LEAGUE_PATH, queue, tier, rank);

        self.api.get(path)
    }
}
