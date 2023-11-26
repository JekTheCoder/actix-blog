use actix_web::HttpResponse;
use serde::Serialize;

use crate::modules::db::QueryResult;

pub fn select_response<T: Serialize>(result: Result<T, sqlx::Error>) -> HttpResponse {
    match result {
        Ok(body) => HttpResponse::Ok().json(body),
        Err(e) => match e {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

pub fn void_insert_response(result: Result<QueryResult, sqlx::Error>) -> HttpResponse {
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => match e {
            sqlx::Error::Database(_) => HttpResponse::Conflict().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

pub fn insert_response<T: Serialize>(result: Result<T, sqlx::Error>) -> HttpResponse {
    match result {
        Ok(body) => HttpResponse::Created().json(body),
        Err(e) => match e {
            sqlx::Error::Database(_) => HttpResponse::Conflict().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

pub fn deleted_response(result: Result<QueryResult, sqlx::Error>) -> HttpResponse {
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => match e {
            sqlx::Error::Database(_) => HttpResponse::Conflict().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
