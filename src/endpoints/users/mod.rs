use super::lib::models::{User, AddUserForm, AddUserDB};
use super::lib;
use rocket_contrib::json::Json;

// todo: split out data access from http methods
use diesel::pg::PgConnection;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::SelectDsl;
use bcrypt::{DEFAULT_COST, hash};
use super::lib::schema::users;


#[post("/users", format = "json", data = "<add_user>")]
pub fn add(add_user: Json<AddUserForm>) -> Json<User> {
  let connection = lib::establish_connection();

  return Json(create(add_user.into_inner(), &connection))
}

fn create(add_user: AddUserForm, connection: &PgConnection) -> User {
  let new_user = AddUserDB {
      account_name: add_user.account_name,
      password_hash: hash(add_user.password, DEFAULT_COST).unwrap()
  };

  diesel::insert_into(users::table)
      .values(&new_user)
      .get_result(connection)
      .expect("Error saving new user")
}

#[get("/users/<id>")]
pub fn get(id: i32) -> Json<Vec<User>> {
  let connection = lib::establish_connection();

  return Json(find(id, &connection));
}

fn find(id: i32, connection: &PgConnection) -> Vec<User> {

  let results = users::table.filter(users::id.eq(id))
    // todo: remove password hash if responding over web
    .load::<User>(connection)
    .expect("Error loading user");

  return results;
}
