use actix_web::{post, web::Data, HttpResponse, Responder, ResponseError};

use crate::{
    modules::{
        admin::AdminId,
        blog::{self, BlogParse},
        category,
        db::Pool,
    },
    shared::{extractors::valid_json::ValidJson, models::insert_return::IdSelect},
};

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Request {
    #[validate(length(min = 1))]
    pub content: String,

    #[validate(length(min = 1))]
    pub categories: Vec<uuid::Uuid>,
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
    let Request {
        content,
        categories,
    } = req.into_inner();

    let BlogParse {
        title,
        content: html_content,
    } = blog::parse(&content)?;

    let result = blog::create(pool.get_ref(), id, &title, &content, &html_content).await?;
    let blog_id = match result {
        Some(IdSelect { id }) => id,
        None => return Err(Error::Conflict),
    };

    category::link_sub_categories(pool.get_ref(), categories, blog_id).await?;
    Ok(HttpResponse::Created().finish())
}
