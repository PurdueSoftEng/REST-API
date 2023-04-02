use chrono::NaiveDateTime;

use crate::schema::packages;

#[derive(Serialize, Deserialize, Queryable)]
pub struct PackageView {
    pub id: i64,
    pub view_time: NaiveDateTime,
    pub url: String,
    pub user_agent: String,
    pub device_type: i8,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertablePackage {
    pub url: String,
    pub user_agent: String,
    pub device_type: i8,
}