use futures_util::{FutureExt, TryFutureExt};
use serde::Serialize;
use sqlx::{query, query_as};
use std::future::Future;
use uuid::Uuid;

use crate::{
    db::{Pool, QueryResult},
    error::sqlx::{insert::InsertErr, select::SelectErr},
    models::select_slice::SelectSlice,
    routes::blogs::comments::create_comment::CreateComment,
};

#[derive(Serialize)]
pub struct Reply {
    pub id: Uuid,
    pub user_id: Uuid,
    pub comment_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
}

impl Reply {
    pub fn get_many_by_parent<'a>(
        pool: &'a Pool,
        comment_id: Uuid,
        parent_id: Uuid,
        SelectSlice { limit, offset }: SelectSlice,
    ) -> impl Future<Output = Result<Vec<Reply>, SelectErr>> + 'a {
        query_as!(
            Reply,
            "SELECT id, user_id, comment_id, parent_id, content FROM replies \
                WHERE comment_id = $1 AND parent_id = $2 LIMIT $3 OFFSET $4",
            comment_id,
            parent_id,
            limit,
            offset,
        )
        .fetch_all(pool)
        .map_err(|e| e.into())
    }

    pub fn get_many<'a>(
        pool: &'a Pool,
        comment_id: Uuid,
        SelectSlice { limit, offset }: SelectSlice,
    ) -> impl Future<Output = Result<Vec<Reply>, SelectErr>> + 'a {
        query_as!(
            Reply,
            "SELECT id, user_id, comment_id, parent_id, content FROM replies \
                WHERE comment_id = $1 LIMIT $2 OFFSET $3",
            comment_id,
            limit,
            offset,
        )
        .fetch_all(pool)
        .map_err(|e| e.into())
    }

    pub fn create<'a>(
        pool: &'a Pool,
        req: &'a CreateComment,
        user_id: Uuid,
        comment_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> impl Future<Output = Result<QueryResult, InsertErr>> + 'a {
        query!(
            "INSERT INTO replies (content, user_id, comment_id, parent_id) \
            VALUES ($1, $2, $3, $4)",
            &req.content,
            user_id,
            comment_id,
            parent_id,
        )
        .execute(pool)
        .map(|query| query.map_err(|e| e.into()))
    }
}
