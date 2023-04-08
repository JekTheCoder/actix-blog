use actix_web::{ResponseError, http::StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Internal error")]
pub struct InternalError;

impl ResponseError for InternalError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
