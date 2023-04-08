use actix_web::HttpResponse;
use serde::Serialize;
use sqlx::postgres::PgQueryResult;

pub trait IntoHttp {
    fn into_http(self) -> HttpResponse;
}

impl IntoHttp for Result<PgQueryResult, sqlx::Error> {
    fn into_http(self) -> HttpResponse {
        match self {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(sqlx::Error::RowNotFound) => HttpResponse::Conflict().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl<T: Serialize> IntoHttp for Option<T> {
    fn into_http(self) -> HttpResponse {
        match self {
            Some(t) => HttpResponse::Ok().json(t),
            None => HttpResponse::NotFound().finish(),
        }
    }
}
