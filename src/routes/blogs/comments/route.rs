use crate::{
    modules::{auth::Claims, db::Pool},
    shared::{
        db::models::comments::{self, CommentJoinUser},
        extractors::{partial_query::PartialQuery, valid_json::ValidJson},
        models::select_slice::SelectSlice,
    },
    traits::{created_reponse::CreatedReponse, json_result::JsonResult},
};

use super::response::CommentByBlog;

use actix_web::{
    get, post,
    web::{scope, Data, Path, ServiceConfig},
    Responder,
};
use uuid::Uuid;

#[get("/")]
pub async fn get_all(
    pool: Data<Pool>,
    blog_id: Path<Uuid>,
    slice: PartialQuery<SelectSlice>,
) -> impl Responder {
    comments::by_blog(pool.get_ref(), blog_id.into_inner(), slice.into_inner())
        .await
        .map(|comments| {
            comments
                .into_iter()
                .map(<CommentByBlog as From<CommentJoinUser>>::from)
                .collect::<Vec<_>>()
        })
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
    .created_response()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{blog_id}/comments")
            .service(get_all)
            .service(create),
    );
}
