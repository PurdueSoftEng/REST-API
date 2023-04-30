use diesel::sql_types::Double;

use crate::schema::{packages, groups, users, requests};

use Default;

#[derive(Serialize, Deserialize, Queryable)]
pub struct PackageView {
    pub package_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub version: String,
    pub package_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsprogram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_one: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_two: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_three: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_four: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_five: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_six: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_seven: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_score: Option<f32>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertablePackage {
    pub version: String,
    pub package_name: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertablePackageURL {
    pub url: Option<String>,
    pub version: String,
    pub package_name: String,
    pub jsprogram: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertablePackageContent {
    pub content: Option<String>,
    pub version: String,
    pub package_name: String,
    pub jsprogram: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertableMetrics {
    pub metric_one: f32,
    pub metric_two: f32,
    pub metric_three: f32,
    pub metric_four: f32,
    pub metric_five: f32,
    pub metric_six: f32,
    pub metric_seven: f32,
    pub total_score: f32,
}

#[derive(Default, Debug, Serialize, Deserialize, QueryableByName)]
#[table_name = "packages"]
pub struct GetData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsprogram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, QueryableByName)]
#[table_name = "packages"]
pub struct GetMetaData {
    pub package_id: i32,
    pub version: String,
    pub package_name: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageQuery {
    pub Name: String,
    pub Version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub metadata: PackageMetaData,
    pub data: PackageData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub URL: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub JSProgram: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageMetaData {
    pub ID: String,
    pub Name: String,
    pub Version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub isAdmin: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageRating {
    pub RampUp: Option<f32>,
    pub Correctness: Option<f32>,
    pub BusFactor: Option<f32>,
    pub ResponsiveMaintainer: Option<f32>, 
    pub LicenseScore: Option<f32>,
    pub GoodPinningPractice: Option<f32>,
    pub PullRequest: Option<f32>,
    pub NetScore: Option<f32>,
}