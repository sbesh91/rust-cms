use serde::{Serialize, Deserialize};
use diesel::Queryable;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub account_name: String,
    pub password_hash: String,
}