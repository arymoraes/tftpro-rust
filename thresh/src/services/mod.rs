pub mod league;
pub mod summoner;

use dotenv::dotenv;
use std::env;

pub fn get_api_key() -> String {
    dotenv().ok();

    let database_url = env::var("RIOT_API_KEY").expect("Key does not exist");

    database_url
}
