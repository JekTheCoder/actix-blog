use crate::{traits::into_http_err::IntoHttpErr, error::http::code::HttpCode};

pub enum SelectErr {
    NotFound,
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
