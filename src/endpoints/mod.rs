pub mod auth;
pub mod catchers;
pub mod lib;
pub mod sections;
pub mod users;

pub fn run() {
    lib::establish_connection();

    let e = rocket::ignite()
        .mount(
            "/",
            routes![
                auth::authenticate,
                users::add,
                users::get,
                sections::get,
                sections::add,
                sections::put,
                sections::delete
            ],
        )
        .register(catchers![
            catchers::bad_request,
            catchers::not_found,
            catchers::internal_server_error,
        ])
        .launch();

    println!("Whoops! Rocket didn't launch!");
    println!("This went wrong: {}", e);
}
