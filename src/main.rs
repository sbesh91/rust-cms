#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate bcrypt;
extern crate dotenv;

pub mod endpoints;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    endpoints::run();
}
