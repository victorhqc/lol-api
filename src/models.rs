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
    pub ts: u32,
    /// Champion ID for this entry.
    pub champion_id: u64,
    /// Number of points needed to achieve next level. Zero if player reached maximum champion
    /// level for this champion.
    pub champion_points_until_next_level: u64,
    /// Last time this champion was played by this player - in Unix milliseconds time format.
    #[serde(with = "ts_milliseconds")]
    pub last_play_time: DateTime<Utc>,
    /// The token earned for this champion to levelup.
    pub tokens_earned: u32,
    /// Number of points earned since current level has been achieved.
    pub champion_points_since_last_level: u64,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchlistDTO {
    pub matches: Vec<MatchDTO>,
    pub total_games: u32,
    pub start_index: u32,
    pub end_index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchDTO {
    ///  Please refer to the Game Constants documentation.
    pub season_id: u32,
    /// Please refer to the Game Constants documentation.
    pub queue_id: u64,
    pub game_id: String,
    /// Participant identity information.
    pub participant_identities: Vec<ParticipantIdentityDTO>,
    /// The major.minor version typically indicates the patch the match was played on.
    pub game_version: String,
    /// Platform where the match was played.
    pub platform_id: String,
    /// Please refer to the Game Constants documentation.
    pub game_mode: String,
    /// Please refer to the Game Constants documentation.
    pub map_id: u32,
    /// Please refer to the Game Constants documentation.
    pub game_type: String,
    /// Team information.
    pub teams: Vec<TeamStatsDTO>,
    /// Participant information.
    pub participants: Vec<ParticipantDTO>,
    /// Match duration in seconds.
    pub game_duration: u64,
    /// Designates the timestamp when champion select ended and the loading screen appeared,
    /// NOT when the game timer was at 0:00.
    #[serde(with = "ts_milliseconds")]
    pub game_creation: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentityDTO {
    /// Player information.
    pub player: PlayerDTO,
    pub participant_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDTO {
    pub current_platform_id: String,
    pub summoner_name: String,
    pub match_history_uri: String,
    /// Original platformId.
    pub platform_id: String,
    /// Player's current accountId (Encrypted)
    pub current_account_id: String,
    pub profile_icon: u32,
    /// Player's summonerId (Encrypted)
    pub summoner_id: String,
    /// Player's original accountId (Encrypted)
    pub account_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantDTO {
    /// Participant statistics.
    pub stats: ParticipantStatsDTO,
    pub participant_id: u32,
    /// List of legacy Rune information. Not included for matches played with Runes Reforged.
    pub runes: Vec<RuneDTO>,
    /// Participant timeline data.
    pub timeline: ParticipantTimelineDTO,
    /// 100 for blue side. 200 for red side.
    pub team_id: u8,
    /// Second Summoner Spell id.
    pub spell2_id: u32,
    /// List of legacy Mastery information. Not included for matches played with Runes Reforged.
    pub masteries: Vec<MasteryDTO>,
    /// Highest ranked tier achieved for the previous season in a specific subset of queueIds, if
    /// any, otherwise null. Used to display border in game loading screen. Please refer to the
    /// Ranked Info documentation. (Legal values: CHALLENGER, MASTER, DIAMOND, PLATINUM, GOLD,
    /// SILVER, BRONZE, UNRANKED)
    pub highest_achieved_season_tier: Tier,
    /// First Summoner Spell id.
    pub spell1_id: u32,
    pub champion_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamStatsDTO {
    /// Flag indicating whether or not the team scored the first Dragon kill.
    pub first_dragon: bool,
    /// Flag indicating whether or not the team destroyed the first inhibitor.
    pub first_inhibitor: bool,
    /// If match queueId has a draft, contains banned champion data, otherwise empty.
    pub bans: Vec<TeamBansDTO>,
    /// Number of times the team killed Baron.
    pub baron_kills: u8,
    /// Flag indicating whether or not the team scored the first Rift Herald kill.
    pub first_rift_herald: bool,
    /// Flag indicating whether or not the team scored the first Baron kill.
    pub first_baron: bool,
    /// Number of times the team killed Rift Herald.
    pub rift_herald_kills: u8,
    /// Flag indicating whether or not the team scored the first blood.
    pub first_blood: bool,
    /// 100 for blue side. 200 for red side.
    pub team_id: u8,
    /// Flag indicating whether or not the team destroyed the first tower.
    pub first_tower: bool,
    /// Number of times the team killed Vilemaw.
    pub vilemaw_kills: u8,
    /// Number of inhibitors the team destroyed.
    pub inhibitor_kills: u8,
    /// Number of towers the team destroyed.
    pub tower_kills: u8,
    /// For Dominion matches, specifies the points the team had at game end.
    pub dominion_bictory_score: u32,
    /// String indicating whether or not the team won. There are only two values visibile in public
    /// match history. (Legal values: Fail, Win)
    pub win: String,
    /// Number of times the team killed Dragon.
    pub dragon_kills: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamBansDTO {
    /// Turn during which the champion was banned.
    pub pick_turn: u8,
    /// Banned championId.
    pub champion_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantStatsDTO {
    pub first_blood_assist: bool,
    pub vision_score: u64,
    pub magic_damage_dealt_to_champions: u64,
    pub damage_dealt_to_objectives: u64,
    pub total_time_crowd_control_dealt: u32,
    pub u64est_time_spent_living: u32,
    /// Post game rune stats.
    pub perk1_var1: u32,
    /// Post game rune stats.
    pub perk1_var3: u32,
    /// Post game rune stats.
    pub perk1_var2: u32,
    pub triple_kills: u32,
    /// Post game rune stats.
    pub perk3_var3: u32,
    pub node_neutralize_assist: u32,
    /// Post game rune stats.
    pub perk3_var2: u32,
    pub player_score9: u32,
    pub player_score8: u32,
    pub kills: u32,
    pub player_score1: u32,
    pub player_score0: u32,
    pub player_score3: u32,
    pub player_score2: u32,
    pub player_score5: u32,
    pub player_score4: u32,
    pub player_score7: u32,
    pub player_score6: u32,
    /// Post game rune stats.
    pub perk_5var1: u32,
    /// Post game rune stats.
    pub perk_5var3: u32,
    /// Post game rune stats.
    pub perk_5var2: u32,
    pub total_score_rank: u32,
    pub neutral_minions_killed: u32,
    pub damage_dealt_to_turrets: u64,
    pub physical_damage_dealt_to_champions: u64,
    pub node_capture: u32,
    pub largest_multi_kill: u32,
    /// Post game rune stats.
    pub perk2_var2: u32,
    /// Post game rune stats.
    pub perk2_var3: u32,
    pub total_units_healed: u32,
    /// Post game rune stats.
    pub perk2_var1: u32,
    /// Post game rune stats.
    pub perk4_var1: u32,
    /// Post game rune stats.
    pub perk4_var2: u32,
    /// Post game rune stats.
    pub perk4_var3: u32,
    pub wards_killed: u32,
    pub largest_critical_strike: u32,
    pub largest_killing_spree: u32,
    pub quadra_kills: u32,
    pub team_objective: u32,
    pub magic_damage_dealt: u64,
    pub item2: u32,
    pub item3: u32,
    pub item0: u32,
    pub neutral_minions_killed_team_jungle: u32,
    pub item6: u32,
    pub item4: u32,
    pub item5: u32,
    /// Primary path rune.
    pub perk1: u32,
    /// Primary path keystone rune.
    pub perk0: u32,
    /// Primary path rune.
    pub perk3: u32,
    /// Primary path rune.
    pub perk2: u32,
    /// Secondary path rune.
    pub perk5: u32,
    /// Secondary path rune.
    pub perk4: u32,
    /// Post game rune stats.
    pub perk3_var1: u32,
    pub damage_self_mitigated: u64,
    pub magical_damage_taken: u64,
    pub first_inhibitor_kill: bool,
    pub true_damage_taken: u64,
    pub node_neutralize: u32,
    pub assists: u32,
    pub combat_player_score: u32,
    /// Primary rune path
    pub perk_primary_style: u32,
    pub gold_spent: u32,
    pub true_damage_dealt: u64,
    pub participant_id: u32,
    pub total_damage_taken: u64,
    pub physical_damage_dealt: u64,
    pub sight_wards_bought_in_game: u32,
    pub total_damage_dealt_to_champions: u64,
    pub physical_damage_taken: u64,
    pub total_player_score: u32,
    pub win: bool,
    pub objective_player_score: u32,
    pub total_damage_dealt: u64,
    pub item1: u32,
    pub neutral_minions_killed_enemy_jungle: u32,
    pub deaths: u32,
    pub wards_placed: u32,
    /// Secondary rune path
    pub perk_sub_style: u32,
    pub turret_kills: u32,
    pub first_blood_kill: bool,
    pub true_damage_dealt_to_champions: u64,
    pub gold_earned: u32,
    pub killing_sprees: u32,
    pub unreal_kills: u32,
    pub altars_captured: u32,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,
    pub champ_level: u32,
    pub double_kills: u32,
    pub node_capture_assist: u32,
    pub inhibitor_kills: u32,
    pub first_inhibitor_assist: bool,
    /// Post game rune stats.
    pub perk0_var1: u32,
    /// Post game rune stats.
    pub perk0_var2: u32,
    /// Post game rune stats.
    pub perk0_var3: u32,
    pub vision_wards_bought_in_game: u32,
    pub altars_neutralized: u32,
    pub penta_kills: u32,
    pub total_heal: u64,
    pub total_minions_killed: u32,
    pub time_c_c_ing_others: u64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RuneDTO {
    pub rune_id: u32,
    pub rank: Division,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantTimelineDTO {

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MasteryDTO {
    pub mastery_id: u32,
    pub rank: Division,
}
