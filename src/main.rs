#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod other;
mod catchers;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let e = rocket::ignite()
        .mount("/", routes![index, other::some])
        .mount("/", routes![other::blah])
        .register(catchers![catchers::not_found, catchers::internal_server_error])
        .launch();

    println!("Whoops! Rocket didn't launch!");
    println!("This went wrong: {}", e);
}