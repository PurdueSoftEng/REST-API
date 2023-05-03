use diesel::sql_types::Text;
use diesel::{self, prelude::*};

use rocket::http::Status;
use rocket::Response;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;

use crate::models::*;
use crate::schema::{self, packages, groups, users};
use crate::DbConn;
use std::vec::Vec;
use metrics::calcscore;
use zip::read;
use base64::{Engine as _, engine::general_purpose};
use std::io::Read;
use std::io::Cursor;


#[get("/")]
pub fn index(conn: DbConn) -> &'static str {

    "Application successfully started!"
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Howdy, world!"
}

#[post("/package/inject", data = "<package>")]
pub fn inject_package(
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

    Ok(format!("Inserted {}.", package.package_name))
}

#[post("/packages?<name>", data = "<query>")]
pub fn query_package(
    conn: DbConn,
    query: Json<PackageQuery>,
    name: String,
) -> Result<String, String> {
    use crate::schema::packages::dsl::*;
    /*let insertable_package = InsertablePackage{package_name: package.Name.clone(), version: package.Version.clone()};
    let inserted_rows = diesel::insert_into(packages)
        .values(&insertable_package)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;*/
    let mut get_meta: Json<Vec<GetMetaData>> = diesel::sql_query("SELECT id, version, package_name FROM packages LIMIT $1")
    .bind::<diesel::sql_types::Integer, _>(name.parse::<i32>().unwrap())
    .get_results(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying packages: {:?}", err);
        "Error querying packages from the database".into()
    })
    .map(Json)?;
    Ok(format!("I found these entries: {:?}", get_meta.0))
}

#[post("/package", data = "<package>")]
pub fn create_package(
    conn: DbConn,
    package: Json<PackageData>,
) -> Result<String, String> {
    use crate::schema::packages::dsl::*;
    /*let binding = package.Content.clone().unwrap();
    let mut wrapped_reader = Cursor::new(binding.as_bytes());
    let mut decoder = base64::read::DecoderReader::new(
        &mut wrapped_reader,
        &general_purpose::STANDARD_NO_PAD);

    // handle errors as you normally would
    let mut result = Vec::new();
    let decoded1 = decoder.read_to_end(&mut result).unwrap();
    let decoded = general_purpose::STANDARD_NO_PAD.decode(package.Content.clone().unwrap().as_bytes()).unwrap();*/
    if package.Content.is_some() || package.URL.is_some()
    {
        let insertable_package1 = InsertablePackageURL{id: "temp".to_string(), package_name: "Temp Name".to_string(), version: "1.0.0".to_string(), url: Some(package.URL.clone().unwrap_or_default()), jsprogram: Some(package.JSProgram.clone().unwrap_or_default())};
        diesel::insert_into(packages)
        .values(&insertable_package1)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;
    
    
        Ok(format!("Inserted Temp Name"))
    }
    else 
    {
        Err("Invalid Format".to_string())
    }
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
        Ok(format!("No Packages"))
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
    let mut get_meta: Json<Vec<GetMetaData>> = diesel::sql_query("SELECT * FROM packages WHERE id=$1").bind::<diesel::sql_types::Text, _>(id.as_str())
    .get_results(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying package {}: {:?}", id, err);
        "Error querying packages from the database".into()
    })
    .map(Json)?;
    let mut get_data: Json<Vec<GetData>> = diesel::sql_query("SELECT * FROM packages WHERE id=$1").bind::<diesel::sql_types::Text, _>(id.as_str())
    .get_results(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying package {}: {:?}", id, err);
        "Error querying packages from the database".into()
    })
    .map(Json)?;
    if get_meta.0.is_empty()
    {
        Ok(format!("Package Not Registered"))
    }
    else 
    {
        Ok(format!("Package Registered"))
    }
    //let meta = PackageMetaData{ID: get_meta.0.pop().unwrap().package_id.to_string(), Name: get_meta.0.pop().unwrap().package_name, Version: get_meta.0.pop().unwrap().version};
    /*if get_data.0.pop().unwrap().content.is_some() && get_data.0.pop().unwrap().jsprogram.is_some() && get_data.0.pop().unwrap().url.is_some()
    {
        //let data = PackageData{Content: Some(get_data.0.pop().unwrap().content.unwrap()), URL: Some(get_data.0.pop().unwrap().url.unwrap()), JSProgram: Some(get_data.0.pop().unwrap().jsprogram.unwrap())};        
    }*/
    //let data = PackageData{Content: Some(get_data.0.pop().unwrap().content.unwrap_or_default()), URL: Some(get_data.0.pop().unwrap().url.unwrap_or_default()), JSProgram: Some(get_data.0.pop().unwrap().jsprogram.unwrap_or_default())};
    //let pack = Package{metadata: meta, data: PackageData{Content: "", URL: "", JSProgram: ""}};
    //Ok(serde_json::to_string(&pack).map(Json).unwrap().0)
}

#[put("/package/<id>", data = "<package>")]
pub fn update_package(conn: DbConn, id: String, package: Json<Package>,) -> Result<String, String>  {
    use crate::schema::packages::dsl::*;
    /*let mut get_res = diesel::sql_query("INSERT INTO packages (package_name, version, content, url, jsprogram) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO UPDATE SET version=$2 SET url=$4")
    .bind::<diesel::sql_types::Text, _>(id.as_str())
    .bind::<diesel::sql_types::Text, _>(package.metadata.Version.as_str())
    .bind::<diesel::sql_types::Text, _>(package.data.Content.as_str())
    .bind::<diesel::sql_types::Text, _>(package.data.URL.as_str())
    .bind::<diesel::sql_types::Text, _>(package.data.JSProgram.as_str())
    .execute(&conn.0)
    .map_err(|err| -> String {
    println!("Error querying package {}: {:?}", id, err);
    "Error querying packages from the database".into()
    })
    .map(Json)?;*/
    Ok(format!("updated rows."))
}

#[delete("/package/<id>")]
pub fn delete_package(conn: DbConn, id: String) -> Result<String, String>  
{
    use crate::schema::packages::dsl::*;
    Ok(format!("Not supported"))
}

#[get("/package/<id>/rate", rank = 2)]
pub fn get_rating(conn: DbConn, id: String) -> Result<String, String>  {
    let mut get_meta: Json<Vec<GetMetaData>> = diesel::sql_query("SELECT * FROM packages WHERE id=$1").bind::<diesel::sql_types::Text, _>(id.as_str())
    .get_results(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying package {}: {:?}", id, err);
        "Error querying packages from the database".into()
    })
    .map(Json)?;
    let mut get_data: Json<Vec<GetData>> = diesel::sql_query("SELECT * FROM packages WHERE id=$1").bind::<diesel::sql_types::Text, _>(id.as_str())
    .get_results(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying package {}: {:?}", id, err);
        "Error querying packages from the database".into()
    })
    .map(Json)?;
    print!("{:?}", get_data.0);
    let meta = PackageMetaData{ID: get_meta.0.pop().unwrap().package_id.to_string(), Name: get_meta.0.pop().unwrap().package_name, Version: get_meta.0.pop().unwrap().version};
    /*if get_data.0.pop().unwrap().content.is_some() && get_data.0.pop().unwrap().jsprogram.is_some() && get_data.0.pop().unwrap().url.is_some()
    {
        //let data = PackageData{Content: Some(get_data.0.pop().unwrap().content.unwrap()), URL: Some(get_data.0.pop().unwrap().url.unwrap()), JSProgram: Some(get_data.0.pop().unwrap().jsprogram.unwrap())};        
    }*/
    let data = PackageData{Content: Some(get_data.0.pop().unwrap().content.unwrap_or_default()), URL: Some(get_data.0.pop().unwrap().url.unwrap_or_default()), JSProgram: Some(get_data.0.pop().unwrap().jsprogram.unwrap_or_default())};
    let pack = Package{metadata: meta, data: data};
    Ok(serde_json::to_string(&pack).map(Json).unwrap().0)
}

#[post("/package/byRegEx", data = "<regex>")]
pub fn search_regex(conn: DbConn, regex: Json<PackageRegEx>) -> Result<String, String>
{
    use crate::schema::packages::dsl::*;
    Ok(format!("Not supported"))
}

#[get("/package/byName/<name>", rank = 1)]
pub fn get_hist(conn: DbConn, name: String) -> Result<String, String>
{
    use crate::schema::packages::dsl::*;
    Ok(format!("Not supported"))
}

#[delete("/package/byName/<name>")]
pub fn delete_hist(conn: DbConn, name: String) -> Result<String, String>
{
    use crate::schema::packages::dsl::*;
    Ok(format!("Not supported"))
}

#[put("/authenticate")]
pub fn auth(conn: DbConn) -> Result<String, String> {
    Err(Status::NotImplemented)
        .map_err(|status| {
            let message = format!("Error handling request: {:?}", status);
            String::from(message)
        })
        .map(|string: String| "This system does not support authentication.".into())
}

#[delete("/reset")]
pub fn reset(conn: DbConn)
{
    //Response::build().raw_status(331, "An Error")
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