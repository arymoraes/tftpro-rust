use diesel::PgConnection;
use serde::Deserialize;

use crate::{diesel::RunQueryDsl, schema::matches_participants_augments};

#[derive(Queryable, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchParticipantAugment {
    pub match_participant_id: i32,
    pub augment_id: i32,
}

#[derive(Insertable)]
#[table_name = "matches_participants_augments"]
pub struct NewMatchParticipantAugment {
    pub match_participant_id: i32,
    pub augment_id: i32,
}

impl NewMatchParticipantAugment {
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
