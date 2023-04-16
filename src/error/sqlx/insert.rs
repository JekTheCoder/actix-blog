use crate::{error::http::code::HttpCode, traits::into_http_err::IntoHttpErr};

pub enum InsertErr {
    NoInsert,
    Unknown,
}

impl From<sqlx::Error> for InsertErr {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(_) => Self::NoInsert,
            _ => Self::Unknown,
        }
    }
}

impl IntoHttpErr for InsertErr {
    type Err = HttpCode;
    fn http_err(self) -> Self::Err {
        match self {
            Self::NoInsert => HttpCode::conflict(),
            Self::Unknown => HttpCode::internal_error(),
        }
    }
}
