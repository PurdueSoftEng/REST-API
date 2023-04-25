use chrono::NaiveDateTime;

use crate::schema::packages;

#[derive(Serialize, Deserialize, Queryable)]
pub struct PackageView {
    pub id: i32,
    pub link: String,
    pub package_name: String,
    pub metric_one: i32,
    pub metric_two: i32,
    pub metric_three: i32,
    pub metric_four: i32,
    pub metric_five: i32,
    pub metric_six: i32,
    pub metric_seven: i32,
    pub total_score: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertablePackage {
    pub link: String,
    pub package_name: String,
    pub metric_one: i32,
    pub metric_two: i32,
    pub metric_three: i32,
    pub metric_four: i32,
    pub metric_five: i32,
    pub metric_six: i32,
    pub metric_seven: i32,
    pub total_score: i32,
}


