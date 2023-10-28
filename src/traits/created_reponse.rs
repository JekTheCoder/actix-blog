use actix_web::HttpResponse;

use crate::error::sqlx::insert::InsertErr;

use super::into_http_err::IntoHttpErr;

pub trait CreatedReponse {
    fn created_response(self) -> Result<HttpResponse, actix_web::Error>;
}

impl<T> CreatedReponse for Result<T, InsertErr> {
    fn created_response(self) -> Result<HttpResponse, actix_web::Error> {
        match self {
            Ok(_) => Ok(HttpResponse::Created().finish()),
            Err(e) => Err(e.http_err().into()),
        }
    }
}
