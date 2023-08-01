use crate::{
    shared::{
        db::{models::comments, Pool},
        extractors::{auth::AuthUser, partial_query::PartialQuery},
        models::select_slice::SelectSlice,
    },
    traits::{catch_http::CatchHttp, into_response::IntoResponse, json_result::JsonResult},
};
use actix_web::{
    get, post,
    web::{scope, Data, Json, Path, ServiceConfig},
    Responder,
};
use uuid::Uuid;
use validator::Validate;

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
    user: AuthUser,
    req: Json<comments::CreateComment>,
) -> actix_web::Result<impl Responder> {
    req.validate().catch_http()?;

    comments::create(
        pool.get_ref(),
        &req,
        user.into_inner().id,
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
