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
    pub account_id: Uuid,
    pub blog_id: Uuid,
    pub content: String,
}

pub struct CommentJoinUser {
    pub id: Uuid,
    pub blog_id: Uuid,
    pub content: String,
    pub account_id: Uuid,
    pub account_name: String,
    pub account_username: String,
}

pub async fn by_blog<'a>(
    pool: &'a Pool,
    blog_id: Uuid,
    SelectSlice { limit, offset }: SelectSlice,
) -> Result<Vec<CommentJoinUser>, SelectErr> {
    let data = query_as!(
        CommentJoinUser,
        r#"SELECT 
            c.id, c.blog_id, c.content, 
            a.id as account_id, a.name as account_name, a.username as account_username
            FROM comments c 
            JOIN accounts a on c.account_id = a.id 
            WHERE blog_id = $1 
            LIMIT $2 OFFSET $3"#,
        blog_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await;

    data.map_err(|e| e.into())
}

pub async fn create<'a>(
    pool: &'a Pool,
    req: &'a CreateComment,
    agent_id: Uuid,
    blog_id: Uuid,
) -> Result<QueryResult, InsertErr> {
    query!(
        "INSERT INTO comments (account_id, blog_id, content) VALUES ($1, $2, $3)",
        agent_id,
        blog_id,
        req.content
    )
    .execute(pool)
    .await
    .map_err(|e| e.into())
}
