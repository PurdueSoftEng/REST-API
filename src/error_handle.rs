use std::fmt;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub msg: String,
    pub status_code: u16,
}

impl CustomError {
    pub fn new(status_code: u16, msg: String) -> CustomError {
        CustomError {
            status_code,
            msg,
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let msg = match status_code.as_u16() < 500 {
            true => self.msg.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": msg }))
    }
}