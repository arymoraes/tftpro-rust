use crate::diesel::RunQueryDsl;
use diesel::PgConnection;

use crate::schema::matches_participants_traits;

#[derive(Queryable)]
pub struct MatchParticipantTrait {
    pub id: i32,
    pub match_participant_id: i32,
    pub trait_id: String,
    pub num_units: i32,
    pub tier_current: i32,
    pub tier_total: i32,
    pub style: i32,
}

#[derive(Insertable)]
#[table_name = "matches_participants_traits"]
pub struct NewMatchParticipantTrait {
    pub match_participant_id: i32,
    pub trait_id: String,
    pub num_units: i32,
    pub tier_current: i32,
    pub tier_total: i32,
    pub style: i32,
}

impl NewMatchParticipantTrait {
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
