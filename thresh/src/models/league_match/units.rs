use diesel::PgConnection;

use crate::diesel::RunQueryDsl;

use crate::models::league_match::match_participant::MatchParticipant;
use crate::schema::matches_participants_units;

#[derive(Insertable, Queryable, Associations)]
#[table_name = "matches_participants_units"]
#[belongs_to(MatchParticipant)]
pub struct MatchParticipantUnit {
    pub match_participant_id: i32,
    pub unit_id: String,
    pub rarity: i32,
    pub tier: i32,
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
