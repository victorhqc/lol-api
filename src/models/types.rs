use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use super::{Rank, Tier};

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
    pub chest_granted: bool,
    pub champion_level: u32,
    pub champion_id: u32,
    pub champion_points_until_next_level: u32,
    #[serde(with = "ts_milliseconds")]
    pub last_play_time: DateTime<Utc>,
    pub champion_points_since_last_level: u32,
    summoner_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionRotation {
    pub free_champion_ids: Vec<u32>,
    pub free_chanpion_ids_for_new_players: Vec<u32>,
    pub max_new_player_level: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub summoner_id: String,
    pub summoner_name: String,
    pub league_id: String,
    pub queue_type: String,
    pub hot_streak: String,
    pub wins: u32,
    pub losses: u32,
    pub rank: Rank,
    pub tier: Tier,
    pub league_points: u32,
    pub veteran: bool,
    pub fresh_blood: bool,
    pub inactive: bool,
}