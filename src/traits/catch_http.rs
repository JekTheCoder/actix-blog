use actix_web::ResponseError;

use crate::error::{http::code::HttpCode, insert::InsertError};

pub trait CatchHttp<T, E> {
    type E;
    fn catch_http(self) -> Result<T, Self::E>;
}

impl<T, E> CatchHttp<T, E> for Result<T, E>
where
    E: IntoHttpErr,
{
    type E = E::Err;

    fn catch_http(self) -> Result<T, Self::E> {
        self.map_err(|e| e.http_err())
    }
}

pub trait IntoHttpErr {
    type Err: ResponseError;

    fn http_err(self) -> Self::Err;
}

impl IntoHttpErr for InsertError {
    type Err = HttpCode;

    fn http_err(self) -> Self::Err {
        match self {
            InsertError::NoInsert => HttpCode::conflict(),
            InsertError::Unknown => HttpCode::internal_error(),
        }
    }
}
