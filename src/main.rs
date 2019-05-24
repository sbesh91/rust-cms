#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_json;
extern crate frank_jwt;
extern crate bcrypt;
extern crate dotenv;

pub mod endpoints;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    endpoints::run();
}
