use serde_derive::{Deserialize, Serialize};
use failure::Error;
use hyper::rt::Future;

use super::descriptor::{ToDescriptor, ConstantDescriptor, ConstantEnum};
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
        self.api
            .get(SEASONS_PATH.to_string())
            .and_then(|seasons| {
                Ok(SeasonList {
                    items: seasons
                })
            })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    id: u32,
    season: String,
}

pub struct SeasonList {
    pub items: Vec<Season>,
}

impl ToDescriptor for SeasonList {
    fn descriptor(&self) -> ConstantDescriptor {
        let enums = self.items
            .iter()
            .map(|season| {
                ConstantEnum {
                    key: season.season.clone(),
                    value: season.season.clone(),
                }
            })
            .collect();

        ConstantDescriptor {
            name: String::from("seasons"),
            enums,
        }
    }
}

const SEASONS_PATH: &'static str = "seasons.json";
