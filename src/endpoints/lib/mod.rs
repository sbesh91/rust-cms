use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    println!("DB connection starting \n");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
