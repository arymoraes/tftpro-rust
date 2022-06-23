use crate::diesel::ExpressionMethods;
use crate::schema::matches;
use diesel::sql_types::Timestamp;
use serde::Deserialize;

struct MatchDtoMetadata {
    data_version: String,
    match_id: String,
    participants: Vec<String>,
}

struct MatchDtoParticipantCompanion {
    content_ID: String,
    skin_ID: i32,
    species: String,
}

struct MatchDtoParticipantTraits {
    name: String,
    num_units: i32,
    style: i32,
    tier_current: i32,
    tier_total: i32,
}

struct MatchDtoParticipantUnits {
    character_id: String,
    itemNames: Vec<String>,
    items: Vec<i32>,
    name: String,
    rarirty: i32,
    tier: i32,
}

struct MatchDtoParticipant {
    augments: Vec<String>,
    companion: MatchDtoParticipantCompanion,
    gold_left: i32,
    last_round: i32,
    level: i32,
    placement: i32,
    players_eliminiated: i32,
    puuid: String,
    time_eliminated: i64,
    total_damage_to_players: i32,
    traits: Vec<MatchDtoParticipantTraits>,
    units: Vec<MatchDtoParticipantUnits>,
}

struct MatchDtoInfo {
    game_datetime: i64,
    game_length: i64,
    game_version: String,
    queue_id: i64,
    tft_game_type: String,
    tft_set_number: i64,
    tft_set_core_name: String,
    participants: Vec<MatchDtoParticipant>,
}

struct MatchDto {
    metadata: MatchDtoMetadata,
    info: MatchDtoInfo,
}

#[table_name = "matches"]
#[derive(Queryable, Insertable, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Match {
    match_id: String,
    tft_set_core_name: String,
    game_datetime: Timestamp,
    game_length: i64,
    region: Option<String>,
}

impl From<MatchDto> for Match {
    fn from(dto: MatchDto) -> Match {
        let new_match = Match {
            match_id: dto.metadata.match_id,
            tft_set_core_name: dto.metadata.match_id,
            game_datetime: dto.info.game_datetime,
            game_length: dto.info.game_length,
            region: None,
        };
        new_match
    }
}
