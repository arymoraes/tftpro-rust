use crate::diesel::RunQueryDsl;
use crate::models::league_match::league_match::Match;
use crate::models::summoner::Summoner;
use crate::schema::matches_participants;

use diesel::PgConnection;

#[derive(Insertable)]
#[table_name = "matches_participants"]
pub struct NewMatchParticipant {
    pub match_id: String,
    pub summoner_id: String,
    pub gold_left: i32,
    pub level: i32,
    pub placement: i32,
    pub last_round: i32,
}

#[derive(Queryable, Associations)]
#[table_name = "matches_participants"]
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

impl NewMatchParticipant {
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
