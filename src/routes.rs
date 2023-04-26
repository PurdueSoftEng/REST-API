use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::{InsertablePackage, PackageView, InsertableGroup, GroupView};
use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn index() -> &'static str {
    "Application successfully started!"
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/package", data = "<package>")]
pub fn create_package(
    conn: DbConn,
    package: Json<InsertablePackage>,
) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::packages::table)
        .values(&package.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/package")]
pub fn list_packages(conn: DbConn) -> Result<Json<Vec<PackageView>>, String> {
    use crate::schema::packages::dsl::*;

    packages
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying packages: {:?}", err);
            "Error querying packages from the database".into()
        })
        .map(Json)
}

#[post("/group", data = "<group>")]
pub fn create_group(
    conn: DbConn,
    group: Json<InsertableGroup>,
) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::groups::table)
        .values(&group.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/group")]
pub fn list_groups(conn: DbConn) -> Result<Json<Vec<GroupView>>, String> {
    use crate::schema::groups::dsl::*;

    groups
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying groups: {:?}", err);
            "Error querying groups from the database".into()
        })
        .map(Json)
}