use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use super::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub account_name: String,
  pub password_hash: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct PublicUser {
  pub id: i32,
  pub account_name:  String,
}

pub trait UserTrait {
  fn account(&self) -> String;
}

impl UserTrait for PublicUser {

  fn account(&self) -> String {
    format!("public user name: {}", self.account_name.to_owned())
  }  
}

impl UserTrait for User {

  fn account(&self) -> String {
    format!("private password hash: {}", self.password_hash.to_owned())
  }
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