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
    pub name: String,
    pub img: Option<String>,
    pub loadouts_icon: String,
    pub guid: String,
}

impl Item {
    pub fn from_name_id(name: &str, conn: &PgConnection) -> Option<i32> {
        use crate::schema::items::dsl::*;

        let query = items.filter(name_id.eq(name)).load::<Item>(conn);

        match query {
            Ok(i) => {
                if i.len() > 0 {
                    return Some(i[0].id);
                } else {
                    return None;
                }
            }
            Err(e) => {
                panic!("Problem while getting item from name_id: {}. \n", e);
            }
        };
    }
}
