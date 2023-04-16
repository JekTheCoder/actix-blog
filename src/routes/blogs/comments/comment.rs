use std::future::Future;

use futures_util::{FutureExt, TryFutureExt};
use serde::Serialize;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{
    db::{Pool, QueryResult},
    error::sqlx::{insert::InsertErr, select::SelectErr},
    models::select_slice::SelectSlice,
};

use super::create_comment::CreateComment;

#[derive(Serialize)]
pub struct Comment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub blog_id: Uuid,
    pub content: String,
}

impl Comment {
    pub fn by_blog<'a>(
        pool: &'a Pool,
        blog_id: Uuid,
        SelectSlice { limit, offset }: SelectSlice,
    ) -> impl Future<Output = Result<Vec<Comment>, SelectErr>> + 'a {
        query_as!(
            Comment,
            "SELECT id, user_id, blog_id, content FROM comments WHERE blog_id = $1 \
            LIMIT $2 OFFSET $3",
            blog_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .map(|query| query.map_err(|e| e.into()))
    }

    pub fn create<'a>(
        pool: &'a Pool,
        req: &'a CreateComment,
        user_id: Uuid,
        blog_id: Uuid,
    ) -> impl Future<Output = Result<QueryResult, InsertErr>> + 'a {
        query!(
            "INSERT INTO comments (user_id, blog_id, content) VALUES ($1, $2, $3)",
            user_id,
            blog_id,
            req.content
        )
        .execute(pool)
        .map_err(|e| e.into())
    }
}
