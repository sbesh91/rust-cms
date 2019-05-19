pub mod catchers;
pub mod auth;
pub mod lib;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn run() {
    lib::establish_connection();
    println!("DB connection started \n");
    
    let e = rocket::ignite()
        .mount("/", routes![index, auth::authenticate])
        .register(catchers![catchers::not_found, catchers::internal_server_error])
        .launch();

    println!("Whoops! Rocket didn't launch!");
    println!("This went wrong: {}", e);
}
