use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use super::constants::{Division, Tier, Queue};

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

/// This object contains single Champion Mastery information for player and champion combination.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMasteryDTO {
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
pub struct ChampionInfo {
    pub free_champion_ids: Vec<u32>,
    pub free_chanpion_ids_for_new_players: Vec<u32>,
    pub max_new_player_level: u32,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeagueEntry {
    pub summoner_id: String,
    pub summoner_name: String,
    pub league_id: String,
    pub queue_type: Queue,
    pub hot_streak: bool,
    pub wins: u32,
    pub losses: u32,
    pub rank: Division,
    pub tier: Tier,
    pub league_points: u32,
    pub veteran: bool,
    pub fresh_blood: bool,
    pub inactive: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeagueItem {
    pub summoner_id: String,
    pub summoner_name: String,
    pub hot_streak: bool,
    pub veteran: bool,
    pub fresh_blood: bool,
    pub inactive: bool,
    pub rank: Division,
    pub league_points: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeagueListDTO {
    pub league_id: String,
    pub tier: Tier,
    pub entries: Vec<LeagueItem>,
    pub queue: Queue,
    pub name: String,
}
