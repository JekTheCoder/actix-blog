use actix_web::web::Data;
use uuid::Uuid;
use sqlx::query_as;

use crate::{persistence::db::{Pool, Slice, DateTime}, server::{service::sync_service, shared::query::QuerySlice}};

sync_service!(GetAll; pool: Data<Pool>);

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogPreview {
    pub id: Uuid,
    pub title: String,
    pub preview: String,
    pub main_image: Option<String>,
    pub created_at: DateTime,
}

impl GetAll {
    pub async fn run(&self, slice: QuerySlice, search: &str) -> Result<Vec<BlogPreview>, sqlx::Error> {
        let Slice { limit, offset } = slice.into();

        query_as!(
            BlogPreview,
            "SELECT id, title, preview, main_image, created_at FROM blogs \
                WHERE title ILIKE $1
                LIMIT $2 OFFSET $3",
            format!("%{}%", search),
            limit,
            offset
        )
        .fetch_all(self.pool.as_ref())
        .await
    }
}
