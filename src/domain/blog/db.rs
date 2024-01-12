use sqlx::query_as;
use uuid::Uuid;

use crate::domain::blog::models::BlogById;
use crate::persistence::db::Pool;

pub async fn by_id(pool: &Pool, id: Uuid) -> Result<BlogById, sqlx::Error> {
    query_as!(
        BlogById,
        "SELECT id, title, html FROM blogs WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}
