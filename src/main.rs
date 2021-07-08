#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;
extern crate bcrypt;
extern crate dotenv;
extern crate frank_jwt;

pub mod endpoints;
use dotenv::dotenv;
use std::process::Command;

fn main() {
    dotenv().ok();

    migrate();

    endpoints::run();
}

fn migrate() {
    //diesel migration run
    let output = Command::new("sh")
        .arg("-c")
        .arg("./migration.sh")
        .output()
        .expect("failed to execute process");

    println!("migration status {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
