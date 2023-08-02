use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::sqlx::{insert::InsertErr, select::SelectErr},
    shared::{
        db::{Pool, QueryResult},
        models::select_slice::SelectSlice,
    },
};

#[derive(Deserialize, Validate)]
pub struct CreateComment {
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Serialize)]
pub struct Comment {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub blog_id: Uuid,
    pub content: String,
}

pub async fn by_blog<'a>(
    pool: &'a Pool,
    blog_id: Uuid,
    SelectSlice { limit, offset }: SelectSlice,
) -> Result<Vec<Comment>, SelectErr> {
    query_as!(
        Comment,
        "SELECT id, agent_id, blog_id, content FROM comments WHERE blog_id = $1 \
            LIMIT $2 OFFSET $3",
        blog_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.into())
}

pub async fn create<'a>(
    pool: &'a Pool,
    req: &'a CreateComment,
    agent_id: Uuid,
    blog_id: Uuid,
) -> Result<QueryResult, InsertErr> {
    query!(
        "INSERT INTO comments (agent_id, blog_id, content) VALUES ($1, $2, $3)",
        agent_id,
        blog_id,
        req.content
    )
    .execute(pool)
    .await
    .map_err(|e| e.into())
}
