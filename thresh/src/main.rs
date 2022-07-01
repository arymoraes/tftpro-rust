use pool::get_connection_pool;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
pub mod pool;
mod schema;
mod services;

fn main() {
    // services::summoner::get_summoners_service();
    // services::league_match::create_matches_service();
    // create a diesel connection pool:
    // let conn = get_connection_pool();
    // services::league::create_leagues_service(&conn.get().expect("Could not connect to DB"));
    services::summoner::get_summoners_service();
}
