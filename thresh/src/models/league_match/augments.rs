use diesel::PgConnection;

use crate::models::league_match::match_participant::MatchParticipant;
use crate::{diesel::RunQueryDsl, schema::matches_participants_augments};

#[derive(Insertable, Queryable, Associations, Clone)]
#[table_name = "matches_participants_augments"]
#[belongs_to(MatchParticipant)]
pub struct MatchParticipantAugment {
    pub match_participant_id: i32,
    pub augment_id: i32,
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
