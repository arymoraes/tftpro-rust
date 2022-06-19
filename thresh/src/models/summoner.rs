use crate::diesel::RunQueryDsl;
use diesel::PgConnection;
use serde::Deserialize;

use crate::schema::summoners;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SummonerRaw {
    pub id: String,
    pub name: String,
    pub profile_icon_id: i32,
    pub summoner_level: i32,
    pub revision_date: i64,
    pub account_id: String,
    pub puuid: String,
}

#[table_name = "summoners"]
#[derive(Queryable, Insertable, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub id: String,
    pub name: String,
    pub region: Option<String>,
    pub league_id: String,
    pub profile_icon_id: i32,
    pub summoner_level: i32,
    pub revision_date: i64,
    pub account_id: String,
    pub puuid: String,
}

impl Summoner {
    pub fn create(&self, conn: &PgConnection) -> () {
        let result: Result<usize, diesel::result::Error> = diesel::insert_into(summoners::table)
            .values(self)
            .execute(conn);

        match result {
            Ok(_) => (),
            Err(e) => println!("Problem while creating summoner: {}", e),
        }
    }
}
