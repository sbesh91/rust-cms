use super::lib::models::{Section};
use super::lib;
use rocket_contrib::json::Json;

// todo: split out data access from http methods
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use super::lib::schema::sections;


#[get("/sections")]
pub fn get() -> Json<Section> {
  let section = Section {
    id: 0,
    module: "".to_string(),
    href: "".to_string(),
    section_type: "".to_string()
  };

  return Json(section);
}