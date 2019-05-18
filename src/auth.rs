use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User { 
  id: i32,
  name: String,
  hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserAuth {
  name: String,
  password: String,
}

#[post("/authenticate", format = "json", data = "<auth>")]
pub fn authenticate(auth: Json<UserAuth>) -> Json<User> {
  // todo get user from db
  let user = User {
    id: 1,
    name: "sbesh91".to_string(),
    hash: "asdf".to_string()
  };


  return Json(user);
}