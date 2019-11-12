use chrono::prelude::*;
use hyper::rt::{Future, Stream};
use hyper::{Method, Request, Body};
use failure::Error;
use serde_derive::{Deserialize, Serialize};
use crate::regions::{WithHosts};

use crate::api::{Api};

pub struct SummonerApi<'a, T> {
    api: &'a Api<T>,
}

impl<'a, T> SummonerApi<'a, T> where T: WithHosts {
    pub fn by_name(&self, name: &String) -> impl Future<Item=Summoner, Error=Error> {
        let path = get_summoner_path("/by-name", name);
        let req = self.api.build_request(Method::GET, path);
        self.get_summoner(req.unwrap())
    }

    pub fn get_summoner(
        &self,
        req: Request<Body>,
    ) -> impl Future<Item=Summoner, Error=Error> {
        self.api.client
            .request(req)
            .and_then(|res| {
                res.into_body().concat2()
            })
            .from_err()
            .and_then(|body| {
                let summoner = serde_json::from_slice(&body)?;

                Ok(summoner)
            })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    id: String,
    account_id: String,
    puuid: String,
    name: String,
    profile_icon_id: u16,
    revision_date: u32,
    summoner_level: NaiveDateTime,
}

pub const SUMMONER_API_PATH: &'static str = "/lol/summoner/v4/summoners";
fn get_summoner_path(route: &str, param: &String) -> String {
    String::from(format!("{}{}{}", SUMMONER_API_PATH, route, param))
}
