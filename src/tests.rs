use super::rocket;
use rocket::local::Client;
use rocket::http::Status;
use rocket::http::ContentType;
use crate::models::*;

#[test]
fn hello_world() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/hello").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Howdy, world!".into()));
}

#[test]
fn post_package() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.post("/package").header(ContentType::JSON).body(r#"{"Content": "abcdefg", "URL": "https://abcdefg.com", "JSProgram":"*.exe"}"#).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Inserted Temp Name.".to_string()))
}

#[test]
fn post_package_url() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.post("/package").header(ContentType::JSON).body(r#"{"URL": "https://abcdefg.com", "JSProgram":"*.exe"}"#).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Inserted Temp Name.".to_string()))
}

#[test]
fn post_package_content() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.post("/package").header(ContentType::JSON).body(r#"{"Content": "abcdefg", "JSProgram":"*.exe"}"#).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Inserted Temp Name.".to_string()))
}

#[test]
fn post_package_invalid() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.post("/package").header(ContentType::JSON).body(r#"{"JSProgram":"*.exe"}"#).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Invalid Format.".to_string()))
}

#[test]
fn get_package() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/package").dispatch();
    assert_eq!(response.status(), Status::Ok);
}