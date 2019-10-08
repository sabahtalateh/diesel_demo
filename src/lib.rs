#[macro_use] extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}

