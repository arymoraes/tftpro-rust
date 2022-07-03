use crate::diesel::RunQueryDsl;
use diesel::PgConnection;

use crate::schema::matches_participants_unit_items;

#[derive(Queryable)]
pub struct MatchParticipantUnitItem {
    pub id: i32,
    pub match_participant_unit_id: i32,
    pub item_id: i32,
}

#[derive(Insertable)]
#[table_name = "matches_participants_unit_items"]
pub struct NewMatchParticipantUnitItem {
    pub match_participant_unit_id: i32,
    pub item_id: i32,
}

impl NewMatchParticipantUnitItem {
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
