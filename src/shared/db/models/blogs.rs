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

pub async fn create(
    pool: &Pool,
    title: &str,
    content: &str,
    admin_id: Uuid,
) -> Result<QueryResult, InsertErr> {
    query!(
        "INSERT INTO blogs(admin_id, title, content) VALUES($1, $2, $3)",
        admin_id,
        title,
        content
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
