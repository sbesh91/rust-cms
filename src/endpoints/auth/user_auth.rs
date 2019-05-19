use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UserAuth {
  name: String,
  password: String,
}