use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::{InsertablePackage, PackageView, InsertableGroup, GroupView};
use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn index() -> &'static str {
    "Application successfully started!"
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