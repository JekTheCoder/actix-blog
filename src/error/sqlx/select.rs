use crate::{error::http::code::HttpCode, traits::into_http_err::IntoHttpErr};
use actix_web::{http::StatusCode, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum SelectErr {
    #[error("Resource not found")]
    NotFound,
    #[error("Internal server error")]
    Unknown,
}

impl From<sqlx::Error> for SelectErr {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => SelectErr::NotFound,
            _ => SelectErr::Unknown,
        }
    }
}

impl IntoHttpErr for SelectErr {
    type Err = HttpCode;
    fn http_err(self) -> Self::Err {
        match self {
            Self::NotFound => HttpCode::not_found(),
            Self::Unknown => HttpCode::internal_error(),
        }
    }
}

impl ResponseError for SelectErr {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SelectErr::NotFound => StatusCode::NOT_FOUND,
            SelectErr::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
