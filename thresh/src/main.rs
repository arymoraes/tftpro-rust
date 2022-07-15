use pool::get_connection_pool;
use services::league_match::league_match::{create_matches_service, get_match_participants};

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
pub mod pool;
mod schema;
mod services;

fn main() {
    // let conn = get_connection_pool();
    // services::league::create_leagues_service(&conn.get().expect("Could not connect to DB"));
    // services::summoner::get_summoners_service();
    create_matches_service();
}
