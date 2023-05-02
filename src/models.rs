use diesel::sql_types::Double;

use crate::schema::{packages, groups, users, requests};

use Default;

#[derive(Serialize, Deserialize, Queryable)]
pub struct PackageView {
    pub package_id: i32,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub version: String,
    pub package_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsprogram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramp_up: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bus_factor: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resp_maintain: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_practice: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_score: Option<f32>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertablePackage {
    pub id: String,
    pub version: String,
    pub package_name: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertablePackageURL {
    pub id: String,
    pub url: Option<String>,
    pub version: String,
    pub package_name: String,
    pub jsprogram: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertablePackageContent {
    pub id: String,
    pub content: Option<String>,
    pub version: String,
    pub package_name: String,
    pub jsprogram: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "packages"]
pub struct InsertableMetrics {
    pub ramp_up: f32,
    pub bus_factor: f32,
    pub resp_maintain: f32,
    pub license: f32,
    pub correct: f32,
    pub good_practice: f32,
    pub pull_request: f32,
    pub net_score: f32,
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
pub struct PackageRegEx {
    pub Regex: String,
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