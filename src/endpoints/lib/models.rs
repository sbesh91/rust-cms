use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use super::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub account_name: String,
  pub password_hash: String,
}

#[derive(Deserialize)]
pub struct AddUserForm {
  pub account_name: String,
  pub password: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct AddUserDB {
  pub account_name: String,
  pub password_hash: String,
}