pub mod user_auth;
use super::users;
use super::lib;
use std::env;
use user_auth::UserAuth;
use rocket_contrib::json::Json;
use bcrypt::{verify};
use frank_jwt::{Algorithm, encode, decode, validate_signature};

#[post("/authenticate", format = "json", data = "<auth>")]
pub fn authenticate(auth: Json<UserAuth>) -> String {

  let connection = lib::establish_connection();
  
  let result = users::find_internal(auth.account_name.to_owned(), &connection);
  let valid_user = verify(&auth.password, &result.password_hash);
  println!("valid login? {}", valid_user.unwrap());


  let header = json!({});
  let payload = json!({
    "id": result.id,
    "account_name": result.account_name, 
  });
  let secret = env::var("SECRET")
    .expect("SECRET must be set");

  let jwt = encode(header, &secret, &payload, Algorithm::HS256)
    .expect("JWT generation failed");

  let decoded_jwt = decode(&jwt, &secret, Algorithm::HS256)
    .expect("JWT decoding failed");

  println!("jwt contents: {}, {}", decoded_jwt.0, decoded_jwt.1);

  let valid_jwt = validate_authorization(&jwt);

  println!("valid jwt? {}", valid_jwt);

  return jwt;
}

// todo use this in http guard
pub fn validate_authorization(jwt: &String) -> bool {
  let secret = env::var("SECRET")
    .expect("SECRET must be set");

  validate_signature(jwt, &secret, Algorithm::HS256)
    .expect("JWT validation failed")
}