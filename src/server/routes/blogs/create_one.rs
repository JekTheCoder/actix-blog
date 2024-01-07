use actix_web::{post, web::Data, HttpResponse, Responder, ResponseError};
use pulldown_cmark::CowStr;
use uuid::Uuid;

use crate::{
    domain::{blog, user::value_objects::AdminId},
    persistence::db::{entities::IdSelect, Pool},
    server::shared::query::ValidJson,
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

    pub preview: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] blog::ParseError),
    #[error("Conflict creating blog")]
    Conflict,
    #[error("Database error")]
    Database,

    #[error("Can not infer preview")]
    NoPreview,
}

impl From<blog::features::create_one::Error> for Error {
    fn from(value: blog::features::create_one::Error) -> Self {
        match value {
            blog::features::create_one::Error::Parse(e) => Self::Parse(e),
            blog::features::create_one::Error::NoPreview => Self::NoPreview,
            blog::features::create_one::Error::Conflict => Self::Conflict,
            blog::features::create_one::Error::Database => Self::Database,
            blog::features::create_one::Error::NoPreview => Self::NoPreview,
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::Parse(_) => actix_web::http::StatusCode::BAD_REQUEST,
            Self::Conflict => actix_web::http::StatusCode::CONFLICT,
            Self::Database => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::NoPreview => actix_web::http::StatusCode::BAD_REQUEST,
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
    req: ValidJson<Request>,
    admin_id: AdminId,
    create_one: blog::features::create_one::CreateOne,
) -> Result<impl Responder, Error> {
    let Request {
        content,
        tags,
        preview,
        category_id,
        sub_categories,
    } = req.into_inner();

    let id = create_one
        .run(
            admin_id,
            &content,
            category_id,
            preview,
            tags,
            sub_categories,
        )
        .await?;

    Ok(HttpResponse::Created().json(IdSelect { id }))
}
