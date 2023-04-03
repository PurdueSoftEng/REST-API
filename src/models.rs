use chrono::NaiveDateTime;

use crate::schema::packages;

#[derive(Serialize, Deserialize, Queryable)]
pub struct PackageView {
    pub id: i64,
    pub url: String,
    pub package_name: String,
    pub metric_one: i8,
    pub metric_two: i8,
    pub metric_three: i8,
    pub metric_four: i8,
    pub metric_five: i8,
    pub metric_six: i8,
    pub metric_seven: i8,
    pub total_score: i8,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertablePackage {
    pub url: String,
    pub package_name: String,
    pub metric_one: i8,
    pub metric_two: i8,
    pub metric_three: i8,
    pub metric_four: i8,
    pub metric_five: i8,
    pub metric_six: i8,
    pub metric_seven: i8,
    pub total_score: i8,
}


