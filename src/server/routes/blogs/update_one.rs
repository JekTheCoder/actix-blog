use actix_web::{put, web::Path, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    domain::blog::{
        features::update_one,
        value_objects::{content::ContentBuf, preview::PreviewBuf, sub_categories::SubCategories},
    },
    server::admin::IsAdminFactory,
    server::shared::{domain_validation::domain_valid, query::DomainJson},
};

domain_valid!(pub struct Request {
    content: ContentBuf,
    preview: Option<PreviewBuf>,
    category_id: Uuid,
    tags: Vec<Uuid>,
    sub_categories: SubCategories,
}; UncheckedRequest);

#[put("/{id}/", wrap = "IsAdminFactory")]
pub async fn endpoint(
    id: Path<Uuid>,
    request: DomainJson<Request>,
    update_one: update_one::UpdateOne,
) -> impl Responder {
    let Request {
        content,
        preview,
        category_id,
        tags,
        sub_categories,
    } = request.into_inner();

    match update_one
        .run(
            id.into_inner(),
            content.as_ref(),
            category_id,
            preview.as_deref(),
            tags,
            sub_categories,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(update_one::Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(update_one::Error::Parse(_)) => {
            HttpResponse::BadRequest().body("Can not parse content")
        }
        Err(update_one::Error::Internal) => HttpResponse::InternalServerError().finish(),
        Err(update_one::Error::NoPreview) => {
            HttpResponse::BadRequest().body("Can not infer preview")
        }
    }
}
