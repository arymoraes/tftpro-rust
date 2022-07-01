use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenv::dotenv;
use std::env;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);

    let conn_pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Connected to database");

    conn_pool
}
