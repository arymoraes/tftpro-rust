use crate::diesel::RunQueryDsl;
use diesel::PgConnection;

use crate::schema::leagues;

#[table_name = "leagues"]
#[derive(Queryable, Insertable)]
pub struct League {
    pub league_id: String,
    pub tier: String,
    pub name: String,
    pub queue: String,
}

impl League {
    pub fn create(&self, conn: &PgConnection) -> () {
        diesel::insert_into(leagues::table)
            .values(self)
            .execute(conn)
            .expect("Something went wrong creating a league");
    }
}
