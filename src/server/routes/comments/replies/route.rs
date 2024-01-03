use actix_web::{
    get, post,
    web::{scope, Data, Path, Query, ServiceConfig},
    Responder,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    server::shared::query::QuerySlice,
    domain::{comment::CreateComment, reply},
    persistence::db::Pool,
    shared::extractors::valid_json::ValidJson,
    sqlx::{insert_response, select_response},
};

use crate::domain::auth::Claims;

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
    slice: Query<QuerySlice>,
) -> impl Responder {
    let comment_id = path.into_inner();

    let res = match parent_id.into_inner().parent_id {
        Some(parent_id) => {
            reply::get_many_by_parent(
                pool.get_ref(),
                comment_id,
                parent_id,
                slice.into_inner().into(),
            )
            .await
        }
        None => reply::get_many(pool.get_ref(), comment_id, slice.into_inner().into()).await,
    };

    let result = res.map(|replies| {
        replies
            .into_iter()
            .map(Into::into)
            .collect::<Vec<ReplyByComment>>()
    });

    select_response(result)
}

#[post("/")]
pub async fn create(
    pool: Data<Pool>,
    path: Path<Uuid>,
    Claims { id, .. }: Claims,
    req: ValidJson<CreateComment>,
    parent_id: Query<ParentUuid>,
) -> impl Responder {
    let comment_id = path.into_inner();

    let result = reply::create(
        pool.get_ref(),
        &req.into_inner().content,
        id,
        comment_id,
        parent_id.parent_id,
    )
    .await;

    insert_response(result)
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{comment_id}/replies")
            .service(create)
            .service(get_all),
    );
}
