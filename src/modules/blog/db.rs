use sqlx::query_as;
use uuid::Uuid;

use crate::{
    modules::{
        blog::models::{BlogById, BlogPreview},
        db::Pool,
    },
    shared::models::{insert_return::IdSelect, select_slice::SelectSlice},
};

pub async fn create(
    pool: &Pool,
    admin_id: Uuid,
    title: &str,
    content: &str,
    html: &str,
    category_id: Uuid,
) -> Result<Option<IdSelect>, sqlx::Error> {
    query_as!(
        IdSelect,
        "INSERT INTO blogs(admin_id, title, content, html, category_id) VALUES($1, $2, $3, $4, $5) RETURNING id",
        admin_id,
        title,
        content,
        html,
        category_id,
    )
    .fetch_optional(pool)
    .await
}

pub async fn by_id(pool: &Pool, id: Uuid) -> Result<BlogById, sqlx::Error> {
    query_as!(
        BlogById,
        "SELECT id, title, html FROM blogs WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_all(pool: &Pool, slice: SelectSlice) -> Result<Vec<BlogPreview>, sqlx::Error> {
    let SelectSlice { limit, offset } = slice;

    query_as!(
        BlogPreview,
        "SELECT id, title, SUBSTRING(html, 0, 200) as html, admin_id FROM blogs \
                LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}
