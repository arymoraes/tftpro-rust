#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
pub mod pool;
mod schema;
mod services;

fn main() {
    // let conn = establish_connection();
    // let pool = get_connection_pool();

    // services::league::create_leagues_service(&conn);
    // let league = models::league::League::all(&conn);
    // println!("{:?}", league);
    services::summoner::get_summoners_service();
}
