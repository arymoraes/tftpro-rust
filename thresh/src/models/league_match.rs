use crate::diesel::RunQueryDsl;
use crate::schema::matches;
use diesel::PgConnection;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MatchDtoMetadata {
    data_version: String,
    match_id: String,
    participants: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct MatchDtoParticipantCompanion {
    content_ID: String,
    skin_ID: i32,
    species: String,
}

#[derive(Deserialize, Debug)]
struct MatchDtoParticipantTraits {
    name: String,
    num_units: i32,
    style: i32,
    tier_current: i32,
    tier_total: i32,
}

#[derive(Deserialize, Debug)]
struct MatchDtoParticipantUnits {
    character_id: String,
    itemNames: Vec<String>,
    items: Vec<i32>,
    name: String,
    rarity: i32,
    tier: i32,
}

#[derive(Deserialize, Debug)]
struct MatchDtoParticipant {
    augments: Vec<String>,
    companion: MatchDtoParticipantCompanion,
    gold_left: i32,
    last_round: i32,
    level: i32,
    placement: i32,
    players_eliminated: i32,
    puuid: String,
    time_eliminated: f64,
    total_damage_to_players: i32,
    traits: Vec<MatchDtoParticipantTraits>,
    units: Vec<MatchDtoParticipantUnits>,
}

#[derive(Deserialize, Debug)]
struct MatchDtoInfo {
    game_datetime: i64,
    game_length: f64,
    game_version: String,
    queue_id: i64,
    tft_game_type: String,
    tft_set_number: i64,
    tft_set_core_name: String,
    participants: Vec<MatchDtoParticipant>,
}

#[derive(Deserialize, Debug)]
pub struct MatchDto {
    metadata: MatchDtoMetadata,
    info: MatchDtoInfo,
}

#[table_name = "matches"]
#[derive(Queryable, Insertable, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub match_id: String,
    pub tft_set_core_name: String,
    pub game_datetime: i32,
    pub game_length: i32,
    pub region: Option<String>,
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

impl Match {
    pub fn create(&self, conn: &PgConnection) -> () {
        let result: Result<usize, diesel::result::Error> = diesel::insert_into(matches::table)
            .values(self)
            .execute(conn);

        match result {
            Ok(_) => (),
            Err(e) => println!("Problem while creating league: {}", e),
        }
    }

    // pub fn all(conn: &PgConnection) -> Vec<League> {
    //     leagues::table.load::<League>(conn).unwrap()
    // }
}
