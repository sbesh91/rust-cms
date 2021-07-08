use super::auth::JWT;
use super::lib;
use super::lib::models::{AddUserDB, AddUserForm, PublicUser, User};
use rocket_contrib::json::Json;

// todo: split out data access from http methods
use super::lib::schema::users;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use bcrypt::{hash, DEFAULT_COST};
use diesel::pg::PgConnection;

#[post("/users", format = "json", data = "<add_user>")]
pub fn add(add_user: Json<AddUserForm>) -> Json<PublicUser> {
    let connection = lib::establish_connection();
    let user = create(add_user.into_inner(), &connection);

    return Json(find_http(user.id, &connection));
}

fn create(add_user: AddUserForm, connection: &PgConnection) -> User {
    let new_user = AddUserDB {
        account_name: add_user.account_name,
        password_hash: hash(add_user.password, DEFAULT_COST).unwrap(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving new user")
}

#[get("/users/<id>")]
pub fn get(id: i32, _key: JWT) -> Json<PublicUser> {
    let connection = lib::establish_connection();

    return Json(find_http(id, &connection));
}

fn find_http(id: i32, connection: &PgConnection) -> PublicUser {
    let results = users::table
        .filter(users::id.eq(id))
        .select((users::id, users::account_name))
        .first::<PublicUser>(connection)
        .expect("Error loading user");

    return results;
}

pub fn find_internal(account_name: String, connection: &PgConnection) -> User {
    let result = users::table
        .filter(users::account_name.eq(account_name))
        .first::<User>(connection)
        .expect("Error loading user");

    return result;
}
