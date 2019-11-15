use failure::Error;
use hyper::rt::Future;

use crate::{
    constants::{Queue, Rank, Tier, WithHost},
    models::{LeagueEntry, LeagueList},
    RiotApi,
};

use super::LEAGUE_PATH;

pub struct LeagueV4<'a> {
    pub api: &'a RiotApi,
}

impl<'a> LeagueV4<'a> {
    pub fn new(api: &'a RiotApi) -> Self {
        Self { api }
    }

    pub fn challenger_leagues_by_queue<T: WithHost>(
        &self,
        platform: T,
        queue: Queue,
    ) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/challengerleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.get(platform, path)
    }

    pub fn grandmaster_leagues_by_queue<T: WithHost>(
        &self,
        platform: T,
        queue: Queue,
    ) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/grandmasterleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.get(platform, path)
    }

    pub fn master_leagues_by_queue<T: WithHost>(
        &self,
        platform: T,
        queue: Queue,
    ) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/masterleagues/by-queue/{}", LEAGUE_PATH, queue);

        self.api.get(platform, path)
    }

    pub fn leagues_by_id<T: WithHost>(
        &self,
        platform: T,
        league_id: &str,
    ) -> impl Future<Item = LeagueList, Error = Error> {
        let path = format!("{}/leagues/{}", LEAGUE_PATH, league_id);

        self.api.get(platform, path)
    }

    pub fn entries_by_summoner_id<T: WithHost>(
        &self,
        platform: T,
        summoner_id: &str,
    ) -> impl Future<Item = Vec<LeagueEntry>, Error = Error> {
        let path = format!("{}/entries/by-summoner/{}", LEAGUE_PATH, summoner_id);

        self.api.get(platform, path)
    }

    pub fn entries<T: WithHost>(
        &self,
        platform: T,
        queue: Queue,
        tier: Tier,
        rank: Rank,
    ) -> impl Future<Item = Vec<LeagueEntry>, Error = Error> {
        let path = format!("{}/entries/{}/{}/{}", LEAGUE_PATH, queue, tier, rank);

        self.api.get(platform, path)
    }
}
