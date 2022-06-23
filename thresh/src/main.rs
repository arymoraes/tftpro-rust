#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
pub mod pool;
mod schema;
mod services;

fn main() {
    // services::summoner::get_summoners_service();
    services::league_match::create_matches_service();
}
