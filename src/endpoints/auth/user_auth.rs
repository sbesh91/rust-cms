use serde::{Deserialize};

#[derive(Deserialize)]
pub struct UserAuth {
  pub account_name: String,
  pub password: String,
}