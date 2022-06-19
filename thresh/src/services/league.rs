use diesel::PgConnection;

use crate::models::league;

pub fn create_league_service<'a>(conn: &PgConnection, title: &'a str) -> league::League {
    let new_league = league::League {
        league_id: String::from("1"),
        name: String::from(title),
        queue: String::from("DANLIMA"),
        tier: String::from("boilimax"),
    };

    new_league.create(conn);

    new_league
}
