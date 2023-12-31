use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error, Serialize, Deserialize)]
#[display(fmt = "{}", cause)]
pub struct GenericError {
    pub code: u16,
    pub cause: String,
}

impl ResponseError for GenericError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(self.code).unwrap()
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl GenericError {
    pub fn new(code: u16, cause: String) -> Self {
        Self { code, cause }
    }
}
