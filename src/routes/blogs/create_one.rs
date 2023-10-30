use actix_web::{post, web::Data, HttpResponse, Responder, ResponseError};

use crate::{
    modules::{
        admin::AdminId,
        blog::{self, BlogParse},
        db::Pool,
    },
    shared::extractors::valid_json::ValidJson,
};

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Request {
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] blog::ParseError),
    #[error("Conflict creating blog")]
    Conflict,
    #[error("Database error")]
    Database,
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::Parse(_) => actix_web::http::StatusCode::BAD_REQUEST,
            Self::Conflict => actix_web::http::StatusCode::CONFLICT,
            Self::Database => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(_) => Self::Conflict,
            _ => Self::Database,
        }
    }
}

#[post("/")]
pub async fn endpoint(
    pool: Data<Pool>,
    req: ValidJson<Request>,
    AdminId { id }: AdminId,
) -> Result<impl Responder, Error> {
    let Request { content } = req.as_ref();

    let BlogParse {
        title,
        content: html_content,
    } = blog::parse(content)?;

    let result = blog::create(pool.get_ref(), id, &title, content, &html_content).await?;

    if result.rows_affected() == 0 {
        return Err(Error::Conflict);
    }

    Ok(HttpResponse::Created().finish())
}
