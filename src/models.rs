use crate::schema::{packages, groups, users, requests};

#[derive(Serialize, Deserialize, Queryable)]
pub struct PackageView {
    pub package_id: i32,
    pub url: String,
    pub version: String,
    pub package_name: String,
    pub jsprogram: String,
    pub metric_one: i32,
    pub metric_two: i32,
    pub metric_three: i32,
    pub metric_four: i32,
    pub metric_five: i32,
    pub metric_six: i32,
    pub metric_seven: i32,
    pub total_score: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "packages"]
pub struct PackageMetaData {
    pub package_id: i32,
    pub url: String,
    pub package_name: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertablePackage {
    pub url: String,
    pub version: String,
    pub package_name: String,
    pub jsprogram: String,
    pub metric_one: i32,
    pub metric_two: i32,
    pub metric_three: i32,
    pub metric_four: i32,
    pub metric_five: i32,
    pub metric_six: i32,
    pub metric_seven: i32,
    pub total_score: i32,
}


#[derive(Serialize, Deserialize, Queryable)]
pub struct GroupView {
    pub group_id: i32,
    pub group_name: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "groups"]
pub struct InsertableGroup {
    pub group_name: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct UserView {
    pub user_id: i32,
    pub group_name: String,
    pub isadmin: bool,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub user_name: String,
    pub isadmin: bool,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct RequestView {
    pub request_id: i32,
    pub request_type: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "requests"]
pub struct InsertableRequest {
    pub request_type: String,
}