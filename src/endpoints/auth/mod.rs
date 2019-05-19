pub mod user_auth;
use super::lib::models::User;
use user_auth::UserAuth;
use rocket_contrib::json::Json;

#[post("/authenticate", format = "json", data = "<auth>")]
pub fn authenticate(auth: Json<UserAuth>) -> Json<User> {
  // todo get user from db
  let user = User {
    id: 1,
    account_name: "sbesh91".to_string(),
    password_hash: "asdf".to_string()
  };

  return Json(user);
}