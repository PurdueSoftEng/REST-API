use actix_web::web::{Json, Path};
use actix_web::HttpResponse;

use serde::{Deserialize, Serialize};
use crate::constants::APPLICATION_JSON;

use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::response::Response;

pub type Resp = Response<R>;

#[derive(Debug, Deserialize, Serialize)]
pub struct R {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
}

impl R {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            message,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RespRequest {
    pub message: Option<String>,
}

impl RespRequest {
    pub fn to_respond(&self) -> Option<R> {
        match &self.message {
            Some(message) => Some(R::new(message.to_string())),
            None => None,
        }
    }
}

#[get("/get")]
pub async fn list() -> HttpResponse {

    let resp = Resp { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(resp)
}

#[post("/post")]
pub async fn create(post_req: Json<RespRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(post_req.to_respond())
}

#[get("/get/{id}")]
pub async fn get(_path: Path<(String,)>) -> HttpResponse {
    let found_id: Option<R> = None;

    match found_id {
        Some(id) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(id),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

#[delete("/delete/{id}")]
pub async fn delete(_path: Path<(String,)>) -> HttpResponse {

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}