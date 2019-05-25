pub mod catchers;
pub mod auth;
pub mod lib;
pub mod users;
pub mod sections;

pub fn run() {
    lib::establish_connection();

    let e = rocket::ignite()
        .mount("/", routes![
            auth::authenticate,
            users::add, users::get,
            sections::get
        ])
        .register(catchers![catchers::not_found, catchers::internal_server_error])
        .launch();

    println!("Whoops! Rocket didn't launch!");
    println!("This went wrong: {}", e);
}
