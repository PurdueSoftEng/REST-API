use diesel::sql_types::Text;
use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::models::{InsertablePackage, PackageView, PackageMetaData, InsertableGroup, GroupView, self};
use crate::schema::{self, packages, groups, users};
use crate::DbConn;
use std::vec::Vec;

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
    use crate::schema::packages::dsl::*;
    let inserted_rows = diesel::insert_into(packages)
        .values(&package.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/package")]
pub fn list_packages(conn: DbConn) -> Result<String, String> {
    use crate::schema::packages::dsl::*;
    let mut package_list: Vec<String> = Vec::new();
    let mut query: Result<Json<Vec<PackageView>>, String> = packages
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying packages: {:?}", err);
            "Error querying packages from the database".into()
        })
        .map(Json);
    if query.as_ref().unwrap().0.is_empty()
    {
        Ok(format!("No Packages", ))
    }
    else
    {
        while !query.as_ref().unwrap().0.is_empty()
        {
            let row = query.as_mut().unwrap().0.pop();
            package_list.push(row.unwrap().package_name);
        }
        Ok(format!("Packages: {:?}", package_list))
    }
}

#[get("/package/<id>")]
pub fn get_package(conn: DbConn, id: String) -> Result<String, String>  {
    let mut result: Json<Vec<PackageMetaData>> = diesel::sql_query("SELECT * FROM packages WHERE package_name=$1").bind::<diesel::sql_types::Text, _>(id.as_str())
    .get_results(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying package {}: {:?}", id, err);
        "Error querying packages from the database".into()
    })
    .map(Json)?;
    Ok(format!("found {:?}.", result.0.pop().unwrap()))
}

#[put("/package/<id>")]
pub fn update_package(conn: DbConn, id: String) -> Result<String, String>  {
    use crate::schema::packages::dsl::*;
    let updated_rows = diesel::update(packages.filter(package_name.eq(id.clone())))
    .set(package_name.eq("New name"))
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying package {}: {:?}", id, err);
        "Error querying packages from the database".into()
    })
    .map(Json)?;
    Ok(format!("updated {}.", updated_rows.0))
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