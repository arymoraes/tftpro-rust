use diesel::PgConnection;

use crate::diesel::RunQueryDsl;

use crate::models::league_match::match_participant::MatchParticipant;
use crate::schema::matches_participants_units;

#[derive(Queryable, Associations)]
#[belongs_to(MatchParticipant)]
#[table_name = "matches_participants_units"]
pub struct MatchParticipantUnit {
    pub id: i32,
    pub match_participant_id: i32,
    pub unit_id: String,
    pub rarity: i32,
    pub tier: i32,
}

#[derive(Insertable)]
#[table_name = "matches_participants_units"]
pub struct NewMatchParticipantUnit {
    pub match_participant_id: i32,
    pub unit_id: String,
    pub rarity: i32,
    pub tier: i32,
}

impl NewMatchParticipantUnit {
    pub fn create(
        &self,
        conn: &PgConnection,
    ) -> Result<MatchParticipantUnit, diesel::result::Error> {
        let result: Result<MatchParticipantUnit, diesel::result::Error> =
            diesel::insert_into(matches_participants_units::table)
                .values(self)
                .get_result(conn);

        match result {
            Ok(new_unit) => Ok(new_unit),
            Err(e) => {
                println!("Problem while creating match participant unit: {}. \n Match ID: {},\n Participant: {}\n", e, self.match_participant_id, self.unit_id);
                Err(e)
            }
        }
    }
}
