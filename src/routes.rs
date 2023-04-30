use diesel::sql_types::Text;
use diesel::{self, prelude::*};

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
pub fn index() -> &'static str {
    "Application successfully started!"
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, world!"
}
//general_purpose::STANDARD_NO_PAD.decode(string).unwrap()
#[post("/packages", data = "<package>")]
pub fn create_packages(
    conn: DbConn,
    package: Json<PackageQuery>,
) -> Result<String, String> {
    use crate::schema::packages::dsl::*;
    let insertable_package = InsertablePackage{package_name: package.Name.clone(), version: package.Version.clone()};
    let inserted_rows = diesel::insert_into(packages)
        .values(&insertable_package)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;
    
    
    Ok(format!("Inserted {} row(s).", inserted_rows))
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
    if package.Content.is_none() && package.URL.is_some()
    {
        let insertable_package1 = InsertablePackageURL{package_name: "Temp Name".to_string(), version: "1.0.0".to_string(), url: Some(package.URL.clone().unwrap_or_default()), jsprogram: Some(package.JSProgram.clone().unwrap_or_default())};
        let inserted_rows1 = diesel::insert_into(packages)
        .values(&insertable_package1)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;
    
    
        Ok(format!("Inserted {} row(s).", inserted_rows1))
    }
    else if package.URL.is_none() && package.Content.is_some() 
    {
        let insertable_package2 = InsertablePackageContent{package_name: "Temp Name".to_string(), version: "1.0.0".to_string(), content: Some(package.Content.clone().unwrap_or_default()), jsprogram: Some(package.JSProgram.clone().unwrap_or_default())};
        let inserted_rows2 = diesel::insert_into(packages)
        .values(&insertable_package2)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;
    
    
        Ok(format!("Inserted {} row(s).", inserted_rows2))
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
    let mut get_meta: Json<Vec<GetMetaData>> = diesel::sql_query("SELECT * FROM packages WHERE package_name=$1").bind::<diesel::sql_types::Text, _>(id.as_str())
    .get_results(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying package {}: {:?}", id, err);
        "Error querying packages from the database".into()
    })
    .map(Json)?;
    let get_data: Json<Vec<GetData>> = diesel::sql_query("SELECT * FROM packages WHERE package_name=$1").bind::<diesel::sql_types::Text, _>(id.as_str())
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
    let data1 = PackageData{Content: None, URL: None, JSProgram: None};
    let pack = Package{metadata: meta, data: data1};
    Ok(serde_json::to_string(&pack).map(Json).unwrap().0)
}

#[put("/package/<id>", data = "<package>")]
pub fn update_package(conn: DbConn, id: String, package: Json<Package>,) -> Result<String, String>  {
    use crate::schema::packages::dsl::*;
    let updated_rows = diesel::update(packages.filter(package_name.eq(id.clone())))
    .set(package_name.eq("new".to_string()))
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying package {}: {:?}", id, err);
        "Error querying packages from the database".into()
    })
    .map(Json)?;
    Ok(format!("updated {}.", updated_rows.0))
}

#[delete("/package/<id>")]
pub fn delete_package(conn: DbConn, id: String) -> Result<String, String>  {
    use crate::schema::packages::dsl::*;
    Ok(format!("Will do"))
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