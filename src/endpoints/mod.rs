pub mod catchers;
pub mod auth;
pub mod lib;
pub mod users;

pub fn run() {
    let e = rocket::ignite()
        .mount("/", routes![
            auth::authenticate,
            users::add, users::get,
        ])
        .register(catchers![catchers::not_found, catchers::internal_server_error])
        .launch();

    println!("Whoops! Rocket didn't launch!");
    println!("This went wrong: {}", e);
}
