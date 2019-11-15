use failure::Error;
use hyper::rt::Future;

use crate::{constants::WithHost, models::ChampionRotation, RiotApi};

use super::CHAMPION_ROTATIONS_PATH;

pub struct ChampionV3<'a> {
    pub api: &'a RiotApi,
}

impl<'a> ChampionV3<'a> {
    pub fn new(api: &'a RiotApi) -> Self {
        Self { api }
    }

    pub fn rotations<T: WithHost>(
        &self,
        platform: T,
    ) -> impl Future<Item = ChampionRotation, Error = Error> {
        self.api.get(platform, CHAMPION_ROTATIONS_PATH.to_string())
    }
}
