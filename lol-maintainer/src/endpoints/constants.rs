use serde_derive::{Deserialize, Serialize};
use failure::Error;
use hyper::rt::Future;

use crate::FetchApi;

pub struct Constants<'a> {
    pub api: &'a FetchApi,
}

impl<'a> Constants<'a> {
    pub fn new(api: &'a FetchApi) -> Self {
        Self { api }
    }

    pub fn get_seasons(
        &self,
    ) -> impl Future<Item = SeasonList, Error = Error> {
        self.api.get(SEASONS_PATH.to_string())
    }
}

pub type SeasonList = Vec<Season>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    id: u32,
    season: String,
}

const SEASONS_PATH: &'static str = "seasons.json";
