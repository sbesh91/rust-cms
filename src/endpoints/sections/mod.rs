use super::auth::JWT;
use super::lib;
use super::lib::models::{AddSectionDB, AddSectionForm, Section, UpdateSectionDB};
use rocket_contrib::json::Json;

// todo: split out data access from http methods
use super::lib::schema::sections;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::SaveChangesDsl;
use crate::diesel::TextExpressionMethods;
use diesel::pg::PgConnection;

#[post("/sections", format = "json", data = "<add_section>")]
pub fn add(add_section: Json<AddSectionForm>, _key: JWT) -> Json<Section> {
    let connection = lib::establish_connection();

    return Json(create(add_section.into_inner(), &connection));
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

    return Json(update(update_section.into_inner(), &connection));
}

fn update(update_section: Section, connection: &PgConnection) -> Section {
    let updated_section = UpdateSectionDB {
        id: update_section.id,
        module: update_section.module,
        href: update_section.href,
        section_type: update_section.section_type,
    };

    updated_section
        .save_changes::<Section>(connection)
        .expect("Error updating old section")
}

#[get("/sections?<section_type>&<href>")]
pub fn get(section_type: Option<String>, href: Option<String>) -> Json<Vec<Section>> {
    let connection = lib::establish_connection();

    let section = section_type.unwrap_or("".to_string());
    let path = href.unwrap_or("".to_string());

    return Json(find(section, path, &connection));
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

#[delete("/sections/<id>")]
pub fn delete(id: i32, _key: JWT) -> Json<usize> {
    let connection = lib::establish_connection();

    return Json(remove(id, &connection));
}

fn remove(id: i32, connection: &PgConnection) -> usize {
    let result = diesel::delete(sections::table.filter(sections::id.eq(id)))
        .execute(connection)
        .expect("Error removing section");

    return result;
}
