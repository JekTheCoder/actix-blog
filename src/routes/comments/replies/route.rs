use actix_web::{
    get, post,
    web::{scope, Data, Path, Query, ServiceConfig},
    Responder,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    shared::{
        db::{
            models::{
                comments::CreateComment,
                replies::{self, ReplyJoinAccount},
            },
            Pool,
        },
        extractors::{partial_query::PartialQuery, valid_json::ValidJson},
        models::select_slice::SelectSlice,
    },
    traits::{created_reponse::CreatedReponse, json_result::JsonResult},
};

use crate::modules::auth::Claims;

use super::response::ReplyByComment;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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

    res.map(|replies| {
        replies
            .into_iter()
            .map(<ReplyByComment as From<ReplyJoinAccount>>::from)
            .collect::<Vec<_>>()
    })
    .json_result()
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
    .created_response()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{comment_id}/replies")
            .service(create)
            .service(get_all),
    );
}
