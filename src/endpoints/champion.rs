use failure::Error;
use hyper::rt::Future;

use crate::{RiotApi, WithHosts};
use crate::models::ChampionRotation;

use super::CHAMPION_ROTATIONS_PATH;

pub struct ChampionV3<'a, T> {
    pub api: &'a RiotApi<T>,
}

impl<'a, T> ChampionV3<'a, T>
where
    T: WithHosts,
{
    pub fn new(api: &'a RiotApi<T>) -> Self {
        Self { api }
    }

    pub fn rotations(&self) -> impl Future<Item = ChampionRotation, Error = Error> {
        self.api
            .get(CHAMPION_ROTATIONS_PATH.to_string())
    }
}
