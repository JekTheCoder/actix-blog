use crate::{
    services::auth::claims::Claims,
    shared::{
        db::{models::comments, Pool},
        extractors::{partial_query::PartialQuery, valid_json::ValidJson},
        models::select_slice::SelectSlice,
    },
    traits::{into_response::IntoResponse, json_result::JsonResult},
};
use actix_web::{
    get, post,
    web::{scope, Data, Path, ServiceConfig},
    Responder,
};
use uuid::Uuid;

use super::replies;

#[get("/")]
pub async fn get_all(
    pool: Data<Pool>,
    blog_id: Path<Uuid>,
    slice: PartialQuery<SelectSlice>,
) -> impl Responder {
    comments::by_blog(pool.get_ref(), blog_id.into_inner(), slice.into_inner())
        .await
        .json_result()
}

#[post("/")]
pub async fn create(
    pool: Data<Pool>,
    blog_id: Path<Uuid>,
    claims: Claims,
    req: ValidJson<comments::CreateComment>,
) -> actix_web::Result<impl Responder> {
    comments::create(
        pool.get_ref(),
        req.as_ref(),
        claims.id,
        blog_id.into_inner(),
    )
    .await
    .into_response()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{blog_id}/comments")
            .service(get_all)
            .service(create)
            .configure(replies::router),
    );
}
