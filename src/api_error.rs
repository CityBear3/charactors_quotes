use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl ApiError {
    pub fn new(error_status_code: u16, error_message: String) -> ApiError {
        ApiError {
            error_status_code,
            error_message,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(error: diesel::result::Error) -> ApiError {
        match error {
            diesel::result::Error::DatabaseError(_, err) => {
                ApiError::new(409, err.message().to_string())
            }
            diesel::result::Error::NotFound => {
                ApiError::new(404, "The record were not found".to_string())
            }
            err => ApiError::new(500, format!("Unknown error: {}", err)),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.error_status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match status_code.as_u16() < 500 {
            true => self.error_message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}
