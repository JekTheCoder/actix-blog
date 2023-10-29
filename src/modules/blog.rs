pub use db::{by_id, create, get_all};
pub use models::{Blog, BlogById, BlogPreview};

mod models {
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Serialize)]
    pub struct Blog {
        pub id: Uuid,
        pub admin_id: Uuid,
        pub title: String,
        pub content: String,
        pub html: String,
    }

    pub struct BlogById {
        pub id: Uuid,
        pub title: String,
        pub html: String,
    }

    #[derive(Serialize)]
    pub struct BlogPreview {
        pub id: Uuid,
        pub admin_id: Uuid,
        pub title: String,
        pub html: Option<String>,
    }
}

mod db {
    use sqlx::{query, query_as};
    use uuid::Uuid;

    use crate::{
        modules::{
            blog::models::{BlogById, BlogPreview},
            db::{Pool, QueryResult},
        },
        shared::models::select_slice::SelectSlice,
    };

    pub async fn create(
        pool: &Pool,
        admin_id: Uuid,
        title: &str,
        content: &str,
        html: &str,
    ) -> Result<QueryResult, sqlx::Error> {
        query!(
            "INSERT INTO blogs(admin_id, title, content, html) VALUES($1, $2, $3, $4)",
            admin_id,
            title,
            content,
            html
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
}
