use diesel::PgConnection;
use serde::Deserialize;

use crate::diesel::RunQueryDsl;
use crate::schema::matches;

#[derive(Queryable, Insertable, Deserialize, Debug)]
#[table_name = "matches"]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub match_id: String,
    pub tft_set_core_name: String,
    pub game_datetime: i32,
    pub game_length: i32,
    pub region: Option<String>,
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
