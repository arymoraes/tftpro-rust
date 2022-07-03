use crate::diesel::RunQueryDsl;
use crate::models::summoner::Summoner;

use crate::schema::matches;
use crate::schema::matches_participants;
use crate::schema::matches_participants_augments;
use crate::schema::matches_participants_traits;
use crate::schema::matches_participants_unit_items;
use crate::schema::matches_participants_units;

use diesel::PgConnection;
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

#[table_name = "matches_participants"]
#[derive(Insertable, Queryable, Associations)]
#[belongs_to(Match)]
#[belongs_to(Summoner)]
pub struct MatchParticipant {
    pub id: i32,
    pub match_id: String,
    pub summoner_id: String,
    pub gold_left: i32,
    pub level: i32,
    pub placement: i32,
    pub last_round: i32,
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

#[table_name = "matches_participants_augments"]
#[derive(Insertable, Queryable, Associations, Clone)]
#[belongs_to(MatchParticipant)]
pub struct MatchParticipantAugment {
    pub match_participant_id: i32,
    pub augment_id: i32,
}

#[table_name = "matches_participants_traits"]
#[derive(Insertable, Queryable, Associations)]
#[belongs_to(MatchParticipant)]
pub struct MatchParticipantTrait {
    pub match_participant_id: i32,
    pub trait_id: String,
    pub num_units: i32,
    pub tier_current: i32,
    pub tier_total: i32,
    pub style: i32,
}

#[table_name = "matches_participants_units"]
#[derive(Insertable, Queryable, Associations)]
#[belongs_to(MatchParticipant)]
pub struct MatchParticipantUnit {
    pub match_participant_id: i32,
    pub unit_id: String,
    pub rarity: i32,
    pub tier: i32,
}

#[table_name = "matches_participants_unit_items"]
#[derive(Insertable, Queryable, Associations)]
#[belongs_to(MatchParticipantUnit)]
pub struct MatchParticipantUnitItem {
    pub match_participant_unit_id: i32,
    pub item_id: i32,
}

impl From<MatchDtoParticipant> for MatchParticipant {
    fn from(dto: MatchDtoParticipant) -> MatchParticipant {
        let new_match_participant = MatchParticipant {
            id: 0,
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
}

impl MatchParticipant {
    pub fn create(&self, conn: &PgConnection) -> () {
        let result: Result<usize, diesel::result::Error> =
            diesel::insert_into(matches_participants::table)
                .values(self)
                .execute(conn);

        match result {
            Ok(_) => (),
            Err(e) => println!("Problem while creating match participant: {}. \n Match ID: {},\n Participant: {}\n", e, self.match_id, self.summoner_id),
        }
    }
}

impl MatchParticipantAugment {
    pub fn create(&self, conn: &PgConnection) -> () {
        let result: Result<usize, diesel::result::Error> =
            diesel::insert_into(matches_participants_augments::table)
                .values(self)
                .execute(conn);

        match result {
            Ok(_) => (),
            Err(e) => println!("Problem while creating match participant augment: {}. \n Match ID: {},\n Participant: {}\n", e, self.match_participant_id, self.augment_id),
        }
    }
}

impl MatchParticipantTrait {
    pub fn create(&self, conn: &PgConnection) -> () {
        let result: Result<usize, diesel::result::Error> =
            diesel::insert_into(matches_participants_traits::table)
                .values(self)
                .execute(conn);

        match result {
            Ok(_) => (),
            Err(e) => println!("Problem while creating match participant trait: {}. \n Match ID: {},\n Participant: {}\n", e, self.match_participant_id, self.trait_id),
        }
    }
}

impl MatchParticipantUnit {
    pub fn create(&self, conn: &PgConnection) -> () {
        let result: Result<usize, diesel::result::Error> =
            diesel::insert_into(matches_participants_units::table)
                .values(self)
                .execute(conn);

        match result {
            Ok(_) => (),
            Err(e) => println!("Problem while creating match participant unit: {}. \n Match ID: {},\n Participant: {}\n", e, self.match_participant_id, self.unit_id),
        }
    }
}

impl MatchParticipantUnitItem {
    pub fn create(&self, conn: &PgConnection) -> () {
        let result: Result<usize, diesel::result::Error> =
            diesel::insert_into(matches_participants_unit_items::table)
                .values(self)
                .execute(conn);

        match result {
            Ok(_) => (),
            Err(e) => println!("Problem while creating match participant unit item: {}. \n Match ID: {},\n Participant: {}\n", e, self.match_participant_unit_id, self.item_id),
        }
    }
}
