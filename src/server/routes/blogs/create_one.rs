use actix_web::{post, HttpResponse, Responder, ResponseError};

use uuid::Uuid;

use crate::{
    domain::{
        blog::{
            self,
            value_objects::{
                content::ContentBuf, preview::PreviewBuf, sub_categories::SubCategories,
            },
        },
        user::admin_id::AdminId,
    },
    persistence::db::entities::IdSelect,
    server::shared::{domain_validation::domain_valid, query::DomainJson},
};

domain_valid!(pub struct Request {
    content: ContentBuf,
    preview: Option<PreviewBuf>,
    category_id: Uuid,
    tags: Vec<Uuid>,
    sub_categories: SubCategories,
}; UncheckedRequest);

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
    req: DomainJson<Request>,
    admin_id: AdminId,
    create_one: blog::features::create_one::CreateOne,
) -> Result<impl Responder, Error> {
    let Request {
        content,
        preview,
        category_id,
        tags,
        sub_categories,
    } = req.into_inner();

    let blog_id = create_one
        .run(
            admin_id,
            content.as_ref(),
            category_id,
            preview.as_deref(),
            tags,
            sub_categories,
        )
        .await?;

    Ok(HttpResponse::Created().json(IdSelect { id: blog_id }))
}
