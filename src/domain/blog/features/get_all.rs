use actix_web::web::Data;
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    persistence::db::{decode::inline_vec::InlineVec, DateTime, Pool, Slice},
    server::{service::sync_service, shared::query::QuerySlice}, domain::category::{category, headless_tag},
};

sync_service!(GetAll; pool: Data<Pool>);

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogPreview {
    pub id: Uuid,
    pub title: String,
    pub preview: String,
    pub main_image: Option<String>,
    pub created_at: DateTime,
    pub category: category::Category,
    pub tags: Vec<headless_tag::HeadlessTag>,
}

pub struct BlogData {
    pub id: Uuid,
    pub title: String,
    pub preview: String,
    pub main_image: Option<String>,
    pub created_at: DateTime,
    pub category_id: Uuid,
    pub category_name: String,
    pub tags: InlineVec<headless_tag::HeadlessTag>,
}

impl From<BlogData> for BlogPreview {
    fn from(data: BlogData) -> Self {
        Self {
            id: data.id,
            title: data.title,
            preview: data.preview,
            main_image: data.main_image,
            created_at: data.created_at,
            category: category::Category {
                id: data.category_id,
                name: data.category_name,
            },
            tags: data.tags.into_inner(),
        }
    }
}

impl GetAll {
    pub async fn run(
        &self,
        slice: QuerySlice,
        search: &str,
    ) -> Result<Vec<BlogPreview>, sqlx::Error> {
        let Slice { limit, offset } = slice.into();

        let blogs = query_as!(
            BlogData,
                r#"SELECT 
                    b.id, b.title, b.preview, b.main_image, c.id as category_id, c.name as category_name, b.created_at, 
                    STRING_AGG(t.id || ',' || t.name || ',' || t.color, ';') AS "tags!: InlineVec<headless_tag::HeadlessTag>"
                FROM blogs b
                JOIN 
                    categories c ON b.category_id = c.id
                JOIN
                    tags_blogs bt ON b.id = bt.blog_id
                JOIN
                    tags t ON bt.tag_id = t.id
                WHERE b.title ILIKE $1
                GROUP BY
                    b.id, c.id
                LIMIT $2 OFFSET $3"#,
            format!("%{}%", search),
            limit,
            offset
        )
        .fetch_all(self.pool.as_ref())
        .await?
        .into_iter()
        .map(BlogPreview::from)
        .collect::<Vec<_>>();

        Ok(blogs)
    }
}
