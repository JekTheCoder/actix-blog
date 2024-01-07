use sqlx::query_as;
use uuid::Uuid;

use crate::domain::blog::models::{BlogById, BlogPreview};
use crate::persistence::db::{Pool, Slice};

pub async fn by_id(pool: &Pool, id: Uuid) -> Result<BlogById, sqlx::Error> {
    query_as!(
        BlogById,
        "SELECT id, title, html FROM blogs WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_all(
    pool: &Pool,
    slice: Slice,
    search: &str,
) -> Result<Vec<BlogPreview>, sqlx::Error> {
    let Slice { limit, offset } = slice;

    query_as!(
        BlogPreview,
        "SELECT id, title, preview, main_image, admin_id FROM blogs \
                WHERE title ILIKE $1
                LIMIT $2 OFFSET $3",
        format!("%{}%", search),
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}
