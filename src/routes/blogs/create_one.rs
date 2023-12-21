use actix_web::{post, web::Data, HttpResponse, Responder, ResponseError};
use uuid::Uuid;

use crate::{
    modules::{
        admin::AdminId,
        blog::{self, BlogParse},
        category,
        db::Pool,
    },
    shared::extractors::valid_json::ValidJson,
};

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[validate(length(min = 1))]
    pub content: String,

    pub category_id: Uuid,
    #[validate(length(min = 1))]
    pub sub_categories: Vec<Uuid>,
    pub tags: Vec<Uuid>,
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
        category_id,
        sub_categories,
        tags,
    } = req.into_inner();

    let BlogParse {
        title,
        content: html_content,
        images,
    } = blog::parse(&content)?;

    let result = blog::create(
        pool.get_ref(),
        id,
        &title,
        &content,
        &html_content,
        category_id,
    )
    .await?;

    let blog_id = match result {
        Some(id) => id,
        None => return Err(Error::Conflict),
    };

    if tags.is_empty() {
        category::link_sub_categories(pool.get_ref(), sub_categories, blog_id.id).await?;
    } else {
        let (categories_result, tags_result) = tokio::join!(
            category::link_sub_categories(pool.get_ref(), sub_categories, blog_id.id),
            category::link_tags(pool.get_ref(), tags, blog_id.id)
        );

        categories_result?;
        tags_result?;
    }

    Ok(HttpResponse::Created().json(blog_id))
}
