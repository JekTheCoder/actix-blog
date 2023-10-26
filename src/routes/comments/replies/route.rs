use actix_web::{
    get, post,
    web::{scope, Data, Path, Query, ServiceConfig},
    Responder,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    services::auth::claims::Claims,
    shared::{
        db::{
            models::{comments::CreateComment, replies},
            Pool,
        },
        extractors::{partial_query::PartialQuery, valid_json::ValidJson},
        models::select_slice::SelectSlice,
    },
    traits::{into_response::IntoResponse, json_result::JsonResult},
};

#[derive(Debug, Deserialize)]
pub struct ParentUuid {
    pub parent_id: Option<Uuid>,
}

#[get("/")]
pub async fn get_all(
    pool: Data<Pool>,
    path: Path<Uuid>,
    parent_id: Query<ParentUuid>,
    slice: PartialQuery<SelectSlice>,
) -> impl Responder {
    let comment_id = path.into_inner();

    let res = match parent_id.into_inner().parent_id {
        Some(parent_id) => {
            replies::get_many_by_parent(pool.get_ref(), comment_id, parent_id, slice.into_inner())
                .await
        }
        None => replies::get_many(pool.get_ref(), comment_id, slice.into_inner()).await,
    };

    res.json_result()
}

#[post("/")]
pub async fn create(
    pool: Data<Pool>,
    path: Path<Uuid>,
    Claims { id, .. }: Claims,
    req: ValidJson<CreateComment>,
    parent_id: Query<ParentUuid>,
) -> actix_web::Result<impl Responder> {
    let comment_id = path.into_inner();

    replies::create(
        pool.get_ref(),
        &req.into_inner().content,
        id,
        comment_id,
        parent_id.parent_id,
    )
    .await
    .into_response()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{comment_id}/replies")
            .service(create)
            .service(get_all),
    );
}
