use actix_web::{http::StatusCode, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("response {0}")]
pub struct HttpCode(StatusCode);

impl ResponseError for HttpCode {
    fn status_code(&self) -> StatusCode {
        self.0.clone()
    }
}

