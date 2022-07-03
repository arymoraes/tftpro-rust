use crate::diesel::RunQueryDsl;
use diesel::PgConnection;

use crate::{
    models::league_match::units::MatchParticipantUnit, schema::matches_participants_unit_items,
};

#[derive(Insertable, Queryable, Associations)]
#[table_name = "matches_participants_unit_items"]
#[belongs_to(MatchParticipantUnit)]
pub struct MatchParticipantUnitItem {
    pub match_participant_unit_id: i32,
    pub item_id: i32,
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
