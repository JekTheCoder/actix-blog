use actix_web::{put, web::Path, HttpResponse, Responder};
use markdown_parse::{content::ContentBuf, preview::PreviewBuf};
use uuid::Uuid;

use crate::{
    domain::blog::features::set_content::{self, SetContent},
    server::admin::IsAdminFactory,
    server::shared::{domain_validation::domain_valid, query::DomainJson},
};

domain_valid!(pub struct Request {
    content: ContentBuf,
    preview: Option<PreviewBuf>,
}; UncheckedRequest);

#[put("/{id}/content", wrap = "IsAdminFactory")]
pub async fn endpoint(
    set_content: SetContent,
    id: Path<Uuid>,
    request: DomainJson<Request>,
) -> impl Responder {
    let Request { content, preview } = request.into_inner();

    match set_content
        .run(id.into_inner(), &content, preview.as_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(set_content::Error::Parse(_)) => {
            HttpResponse::BadRequest().body("Can not parse content")
        }
        Err(set_content::Error::Database) => HttpResponse::InternalServerError().finish(),
        Err(set_content::Error::NoPreview) => {
            HttpResponse::BadRequest().body("Can not infer preview")
        }
    }
}
