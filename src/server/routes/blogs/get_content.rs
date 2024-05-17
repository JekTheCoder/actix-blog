use actix_web::{get, web::Path, HttpResponse};
use uuid::Uuid;

use crate::{domain::blog::features::get_content::GetContent, server::admin::IsAdminFactory};

#[get("/{blog_id}/content.md/", wrap = "IsAdminFactory")]
pub async fn endpoint(get_by_id: GetContent, id: Path<Uuid>) -> HttpResponse {
    match get_by_id.run(id.into_inner()).await {
        Ok(Some(blog)) => HttpResponse::Ok().body(blog),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
