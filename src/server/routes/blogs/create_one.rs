use actix_web::{post, web::Data, HttpResponse, Responder, ResponseError};
use pulldown_cmark::CowStr;
use uuid::Uuid;

use crate::{
    domain::{
        admin::AdminId,
        blog::{self, parse_preview, BlogParse, ImageUrlInjector, ImgHostInjectorFactory},
        category,
    },
    persistence::db::Pool,
    server::shared::query::ValidJson,
    shared::models::insert_return::IdSelect,
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
    pool: Data<Pool>,
    req: ValidJson<Request>,
    AdminId { id: admin_id }: AdminId,
    img_host_injector: ImgHostInjectorFactory,
) -> Result<impl Responder, Error> {
    let Request {
        content,
        category_id,
        sub_categories,
        tags,
        preview,
    } = req.into_inner();

    let id = Uuid::new_v4();
    let injector = img_host_injector.create(id);

    let BlogParse {
        title,
        content: html_content,
        images, // TODO
    } = blog::parse(&content, &injector)?;

    let preview = match preview {
        Some(preview) => preview,
        None => match parse_preview(&content) {
            Some(preview) => preview,
            None => {
                return Err(Error::NoPreview);
            }
        },
    };

    let images = images.into_inner();
    let main_image = images.first().map(|image| {
        let mut cow = CowStr::Borrowed(image);
        injector.inject(&mut cow);

        cow.to_string()
    });

    let result = blog::create(
        pool.get_ref(),
        id,
        admin_id,
        &title,
        &content,
        &html_content,
        category_id,
        &preview,
        main_image.as_deref(),
        &images,
    )
    .await?;

    if result.rows_affected() != 1 {
        return Err(Error::Conflict);
    }

    if tags.is_empty() {
        category::link_sub_categories(pool.get_ref(), sub_categories, id).await?;
    } else {
        let (categories_result, tags_result) = tokio::join!(
            category::link_sub_categories(pool.get_ref(), sub_categories, id),
            category::link_tags(pool.get_ref(), tags, id)
        );

        categories_result?;
        tags_result?;
    }

    Ok(HttpResponse::Created().json(IdSelect { id }))
}
