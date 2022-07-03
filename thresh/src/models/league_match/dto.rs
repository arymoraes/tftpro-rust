use super::match_participant::NewMatchParticipant;
use crate::models::league_match::league_match::Match;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct MatchDtoMetadata {
    data_version: String,
    match_id: String,
    participants: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MatchDtoParticipantCompanion {
    content_ID: String,
    skin_ID: i32,
    species: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MatchDtoParticipantTraits {
    name: String,
    num_units: i32,
    style: i32,
    tier_current: i32,
    tier_total: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MatchDtoParticipantUnits {
    character_id: String,
    itemNames: Vec<String>,
    items: Vec<i32>,
    name: String,
    rarity: i32,
    tier: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MatchDtoParticipant {
    pub augments: Vec<String>,
    pub companion: MatchDtoParticipantCompanion,
    pub gold_left: i32,
    pub last_round: i32,
    pub level: i32,
    pub placement: i32,
    pub players_eliminated: i32,
    pub puuid: String,
    pub time_eliminated: f64,
    pub total_damage_to_players: i32,
    pub traits: Vec<MatchDtoParticipantTraits>,
    pub units: Vec<MatchDtoParticipantUnits>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MatchDtoInfo {
    game_datetime: i64,
    game_length: f64,
    game_version: String,
    queue_id: i64,
    tft_game_type: String,
    tft_set_number: i64,
    tft_set_core_name: String,
    pub participants: Vec<MatchDtoParticipant>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MatchDto {
    pub metadata: MatchDtoMetadata,
    pub info: MatchDtoInfo,
}

impl From<MatchDto> for Match {
    fn from(dto: MatchDto) -> Match {
        let new_match = Match {
            match_id: dto.metadata.match_id,
            tft_set_core_name: dto.info.tft_set_core_name,
            game_datetime: dto.info.game_datetime as i32,
            game_length: dto.info.game_length as i32,
            region: None,
        };
        new_match
    }
}

impl From<MatchDtoParticipant> for NewMatchParticipant {
    fn from(dto: MatchDtoParticipant) -> NewMatchParticipant {
        let new_match_participant = NewMatchParticipant {
            match_id: String::from(""),
            summoner_id: dto.puuid,
            gold_left: dto.gold_left,
            level: dto.level,
            placement: dto.placement,
            last_round: dto.last_round,
        };
        new_match_participant
    }
}
