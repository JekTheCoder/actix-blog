use sqlx::{query, query_as};
use uuid::Uuid;

use crate::modules::{
    blog::models::{BlogById, BlogPreview},
    db::{Pool, QueryResult, Slice},
};

pub async fn create(
    pool: &Pool,
    id: Uuid,
    admin_id: Uuid,
    title: &str,
    content: &str,
    html: &str,
    category_id: Uuid,
    preview: &str,
    main_image: Option<&str>,
    images: &[String],
) -> Result<QueryResult, sqlx::Error> {
    query!(
        r#"INSERT INTO 
blogs(
    id,
    admin_id,
    title,
    content,
    html,
    category_id,
    preview,
    main_image,
    images
) 
VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        id,
        admin_id,
        title,
        content,
        html,
        category_id,
        preview,
        main_image,
        images
    )
    .execute(pool)
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

pub async fn get_all(pool: &Pool, slice: Slice) -> Result<Vec<BlogPreview>, sqlx::Error> {
    let Slice { limit, offset } = slice;

    query_as!(
        BlogPreview,
        "SELECT id, title, preview, main_image, admin_id FROM blogs \
                LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}
