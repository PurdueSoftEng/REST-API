use actix_web::web::{Json, Path};
use actix_web::{delete, get, post, put, web, HttpResponse};

use serde::{Deserialize, Serialize};
use crate::constants::APPLICATION_JSON;

use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::error_handle::CustomError;
use crate::response_handle::CustomResponse;


#[get("/get")]
pub async fn list() -> Result<HttpResponse, CustomError> {
    let resp: CustomResponse = CustomResponse{ status_code: 200, msg: "hello world".to_string()};  
    Ok(HttpResponse::Ok().into())
}

#[post("/post")]
pub async fn create() -> Result<HttpResponse, CustomError> {
    let resp: CustomResponse = CustomResponse{ status_code: 200, msg: "hello world".to_string()}; 
    Ok(HttpResponse::Created().into())
}

#[get("/get/{id}")]
pub async fn get(_path: Path<(String,)>) -> Result<HttpResponse, CustomError> {
    let mut found_id: Option<HttpResponse> = None;

    match found_id {
        Some(id) => Ok(HttpResponse::Ok().into()),
        None => Err(CustomError{status_code: 404, msg: "id not found".to_string()}),
    }
}

#[delete("/delete/{id}")]
pub async fn delete(_path: Path<(String,)>) -> Result<HttpResponse, CustomError> {
    Ok(HttpResponse::NoContent().into())
}