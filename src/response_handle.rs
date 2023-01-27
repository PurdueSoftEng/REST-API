use std::fmt;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomResponse {
    pub status_code: u16,
    pub msg: String,
}

impl CustomResponse {
    pub fn new(status_code: u16, msg: String) -> CustomResponse {
        CustomResponse {
            status_code,
            msg,
        }
    }
}

impl fmt::Display for CustomResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.msg.as_str())
    }
}



