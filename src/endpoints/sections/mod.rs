use super::lib::models::{Section, AddSectionForm, AddSectionDB, UpdateSectionDB};
use super::lib;
use super::auth::{JWT};
use rocket_contrib::json::Json;

// todo: split out data access from http methods
use diesel::pg::PgConnection;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::SaveChangesDsl;
use crate::diesel::QueryDsl;
use crate::diesel::TextExpressionMethods;
use super::lib::schema::sections;

#[post("/sections", format = "json", data = "<add_section>")]
pub fn add(add_section: Json<AddSectionForm>, _key: JWT) -> Json<Section> {
  let connection = lib::establish_connection();

  return Json(create(add_section.into_inner(), &connection))
} 

fn create(add_section: AddSectionForm, connection: &PgConnection) -> Section {
  let new_section = AddSectionDB {
    module: add_section.module,
    href: add_section.href,
    section_type: add_section.section_type,
  };

  diesel::insert_into(sections::table)
    .values(&new_section)
    .get_result(connection)
    .expect("Error saving new section")
}

#[put("/sections", format = "json", data = "<update_section>")]
pub fn put(update_section: Json<Section>, _key: JWT) -> Json<Section> {
  let connection = lib::establish_connection();

  return Json(update(update_section.into_inner(), &connection))
}

fn update(update_section: Section, connection: &PgConnection) -> Section {
  let updated_section = UpdateSectionDB {
    id: update_section.id,
    module: update_section.module,
    href: update_section.href,
    section_type: update_section.section_type,
  };

  updated_section.save_changes::<Section>(connection)
    .expect("Error updating old section")
}


#[get("/sections?<section_type>&<href>")]
pub fn get(section_type: String, href: String) -> Json<Vec<Section>> {
  let connection = lib::establish_connection();

  return Json(find(section_type, href, &connection));
}

fn find(section_type: String, href: String, connection: &PgConnection) -> Vec<Section> {

  let results = sections::table
    .filter(sections::section_type.like(format!("%{}", section_type)))
    .filter(sections::href.like(format!("%{}", href)))
    .order_by(sections::id.desc())
    .load::<Section>(connection)
    .expect("Error loading sections");

  return results;
}
