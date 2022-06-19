use crate::diesel::RunQueryDsl;
use diesel::PgConnection;
use serde::Deserialize;

use crate::schema::leagues;

#[table_name = "leagues"]
#[derive(Queryable, Insertable, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub league_id: String,
    pub tier: String,
    pub name: String,
    pub queue: String,
    pub region: Option<String>,
}

impl League {
    pub fn create(&self, conn: &PgConnection) -> () {
        let result: Result<usize, diesel::result::Error> = diesel::insert_into(leagues::table)
            .values(self)
            .execute(conn);

        match result {
            Ok(_) => (),
            Err(e) => println!("Problem while creating league: {}", e),
        }
    }

    pub fn all(conn: &PgConnection) -> Vec<League> {
        leagues::table.load::<League>(conn).unwrap()
    }
}
