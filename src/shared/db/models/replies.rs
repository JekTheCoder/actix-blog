use serde::Serialize;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{
    error::sqlx::{insert::InsertErr, select::SelectErr},
    shared::{
        db::{Pool, QueryResult},
        models::select_slice::SelectSlice,
    },
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub id: Uuid,
    pub account_id: Uuid,
    pub comment_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub has_replies: bool,
}

pub async fn get_many_by_parent(
    pool: &Pool,
    comment_id: Uuid,
    parent_id: Uuid,
    SelectSlice { limit, offset }: SelectSlice,
) -> Result<Vec<Reply>, SelectErr> {
    query_as!(
        Reply,
        r#"SELECT id, account_id, comment_id, parent_id, content,
            (SELECT COUNT(*) > 0 FROM replies ri WHERE ri.parent_id = ro.id LIMIT 1) as "has_replies!"
            FROM replies ro
            WHERE comment_id = $1 AND parent_id = $2 LIMIT $3 OFFSET $4"#,
        comment_id,
        parent_id,
        limit,
        offset,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.into())
}

pub async fn get_many(
    pool: &Pool,
    comment_id: Uuid,
    SelectSlice { limit, offset }: SelectSlice,
) -> Result<Vec<Reply>, SelectErr> {
    query_as!(
        Reply,
        r#"SELECT id, account_id, comment_id, parent_id, content,
            (SELECT COUNT(*) > 0 FROM replies ri WHERE ri.parent_id = ro.id LIMIT 1) as "has_replies!"
            FROM replies ro
            WHERE comment_id = $1 AND parent_id IS NULL LIMIT $2 OFFSET $3"#,
        comment_id,
        limit,
        offset,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.into())
}

pub async fn create(
    pool: &Pool,
    content: &str,
    account_id: Uuid,
    comment_id: Uuid,
    parent_id: Option<Uuid>,
) -> Result<QueryResult, InsertErr> {
    query!(
        "INSERT INTO replies (content, account_id, comment_id, parent_id) \
            VALUES ($1, $2, $3, $4)",
        content,
        account_id,
        comment_id,
        parent_id,
    )
    .execute(pool)
    .await
    .map_err(|e| e.into())
}
