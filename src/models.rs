use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use super::constants::{Division, Tier, Queue};

/// Represents a summoner
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SummonerDTO {
    /// ID of the summoner icon associated with the summoner.
    pub profile_icon_id: u32,
    /// Summoner name.
    pub name: String,
    /// Encrypted PUUID. Exact length of 78 characters.
    pub puuid: String,
    ///  Summoner level associated with the summoner.
    pub summoner_level: u32,
    /// Date summoner was last modified specified as epoch milliseconds. The following events will
    /// update this timestamp: profile icon change, playing the tutorial or advanced tutorial,
    /// finishing a game, summoner name change
    #[serde(with = "ts_milliseconds")]
    pub revision_date: DateTime<Utc>,
    /// Encrypted summoner ID. Max length 63 characters.
    pub id: String,
    /// Encrypted account ID. Max length 56 characters.
    pub account_id: String,
}

/// This object contains single Champion Mastery information for player and champion combination.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMasteryDTO {
    /// Is chest granted for this champion or not in current season.
    pub chest_granted: bool,
    /// Champion level for specified player and champion combination.
    pub champion_level: u32,
    /// Total number of champion points for this player and champion combination - they are used to
    /// determine championLevel.
    pub champion_points: u32,
    /// Champion ID for this entry.
    pub champion_id: u32,
    /// Number of points needed to achieve next level. Zero if player reached maximum champion
    /// level for this champion.
    pub champion_points_until_next_level: u32,
    /// Last time this champion was played by this player - in Unix milliseconds time format.
    #[serde(with = "ts_milliseconds")]
    pub last_play_time: DateTime<Utc>,
    /// The token earned for this champion to levelup.
    pub tokens_earned: u32,
    /// Number of points earned since current level has been achieved.
    pub champion_points_since_last_level: u32,
    ///  Summoner ID for this entry. (Encrypted)
    pub summoner_id: String,
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
pub struct LeagueEntryDTO {
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
pub struct LeagueItemDTO  {
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
    pub entries: Vec<LeagueItemDTO>,
    pub queue: Queue,
    pub name: String,
}
