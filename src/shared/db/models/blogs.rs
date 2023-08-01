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

#[derive(Serialize)]
pub struct Blog {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct BlogPreview {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub title: String,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateReq {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}

pub async fn create(
    pool: &Pool,
    req: &CreateReq,
    admin_id: Uuid,
) -> Result<QueryResult, InsertErr> {
    query!(
        "INSERT INTO blogs(admin_id, title, content) VALUES($1, $2, $3)",
        admin_id,
        req.title,
        req.content
    )
    .execute(pool)
    .await
    .map_err(|e| e.into())
}

pub async fn by_id(pool: &Pool, id: Uuid) -> Result<Blog, SelectErr> {
    query_as!(
        Blog,
        "SELECT id, title, content, admin_id FROM blogs WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.into())
}

pub async fn get_all(pool: &Pool, slice: SelectSlice) -> Result<Vec<BlogPreview>, SelectErr> {
    let SelectSlice { limit, offset } = slice;

    query_as!(
        BlogPreview,
        "SELECT id, title, SUBSTRING(content, 0, 200) as content, admin_id FROM blogs \
                LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.into())
}
