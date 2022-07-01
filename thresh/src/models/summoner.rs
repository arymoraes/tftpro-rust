use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
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
    pub revision_id: i64,
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

    pub fn exists(id: &str, conn: &PgConnection) -> bool {
        let result: Result<i64, diesel::result::Error> = summoners::table
            .filter(summoners::id.eq(&id))
            .count()
            .get_result(conn); // Result<i64, Error>

        match result {
            Ok(count) => count > 0,
            Err(e) => {
                println!("Problem while checking if summoner exists: {}", e);
                false
            }
        }
    }

    pub fn fetch_by_region(region: &str, conn: &PgConnection) -> Vec<String> {
        let result: Result<Vec<String>, diesel::result::Error> = summoners::table
            .filter(summoners::region.eq(region))
            .select(summoners::puuid)
            .get_results::<String>(conn);

        match result {
            Ok(puuids) => puuids,
            Err(e) => {
                println!("Problem while fetching summoners by region: {}", e);
                Vec::new()
            }
        }
    }
}
