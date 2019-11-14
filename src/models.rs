use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub id: String,
    pub account_id: String,
    pub puuid: String,
    pub name: String,
    pub profile_icon_id: u16,
    pub summoner_level: u16,
    #[serde(with = "ts_milliseconds")]
    pub revision_date: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
    pub chest_granted: bool,
    pub champion_level: u8,
    pub champion_id: u32,
    pub champion_points_until_next_level: u32,
    #[serde(with = "ts_milliseconds")]
    pub last_play_time: DateTime<Utc>,
    pub champion_points_since_last_level: u32,
    summoner_id: String,
}
