use actix_web::{get, web::Path, HttpResponse, Responder, ResponseError};
use uuid::Uuid;

use crate::domain::blog::features::get_by_id::GetById;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Blog not found")]
    NotFound,
    #[error("")]
    Database,
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            Error::Database => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/{blog_id}/")]
pub async fn endpoint(get_by_id: GetById, id: Path<Uuid>) -> Result<impl Responder, Error> {
    match get_by_id.run(id.into_inner()).await {
        Ok(Some(blog)) => Ok(HttpResponse::Ok().json(blog)),
        Ok(None) => Err(Error::NotFound),
        Err(_) => Err(Error::Database),
    }
}
