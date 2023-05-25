pub mod comment;
pub mod replies;

mod create_comment;

use crate::{
    db::Pool,
    extractors::{auth::AuthUser, partial_query::PartialQuery},
    models::select_slice::SelectSlice,
    traits::{catch_http::CatchHttp, into_response::IntoResponse, json_result::JsonResult},
};
use actix_web::{
    get, post,
    web::{scope, Data, Json, Path, ServiceConfig},
    Responder,
};
use comment::Comment;
use uuid::Uuid;
use validator::Validate;

use self::create_comment::CreateComment;

#[get("/")]
pub async fn get_all(
    pool: Data<Pool>,
    blog_id: Path<Uuid>,
    slice: PartialQuery<SelectSlice>,
) -> impl Responder {
    Comment::by_blog(pool.get_ref(), blog_id.into_inner(), slice.into_inner())
        .await
        .json_result()
}

#[post("/")]
pub async fn create(
    pool: Data<Pool>,
    blog_id: Path<Uuid>,
    user: AuthUser,
    req: Json<CreateComment>,
) -> actix_web::Result<impl Responder> {
    req.validate().catch_http()?;

    Comment::create(
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
            .configure(replies::router)
    );
}
