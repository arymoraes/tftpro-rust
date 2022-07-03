use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::schema::items;

use diesel::PgConnection;
use serde::Deserialize;

#[derive(Queryable, Insertable, Deserialize, Debug, Clone)]
#[table_name = "items"]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub name_id: String,
    pub img: Option<String>,
    pub loadouts_icon: String,
    pub guid: String,
}

impl Item {
    pub fn from_name_id(name: &str, conn: &PgConnection) -> Item {
        use crate::schema::items::dsl::*;

        let item = items.filter(name_id.eq(name)).load::<Item>(conn).unwrap();

        item[0].clone()
    }
}
