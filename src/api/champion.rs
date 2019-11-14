use failure::Error;
use hyper::rt::Future;

use crate::hosts::WithHosts;
use crate::models::ChampionRotation;
use crate::Api;

use super::CHAMPION_ROTATIONS_PATH;

pub struct ChampionApi<'a, T> {
    pub api: &'a Api<T>,
}

impl<'a, T> ChampionApi<'a, T>
where
    T: WithHosts,
{
    pub fn new(api: &'a Api<T>) -> Self {
        Self { api }
    }

    pub fn rotations(&self) -> impl Future<Item = ChampionRotation, Error = Error> {
        self.api
            .client_request::<ChampionRotation>(CHAMPION_ROTATIONS_PATH.to_string())
    }
}
