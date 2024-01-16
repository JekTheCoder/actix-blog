use actix_web::web::Data;
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    persistence::db::{decode::inline_vec::InlineVec, DateTime, Pool, Slice},
    server::{service::sync_service, shared::query::QuerySlice}, domain::blog_grouping::{category, headless_tag, headless_sub_category},
};

use headless_sub_category::HeadlessSubCategory;

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
    pub sub_categories: Vec<headless_sub_category::HeadlessSubCategory>,
}

pub struct BlogData {
    pub id: Uuid,
    pub title: String,
    pub preview: String,
    pub main_image: Option<String>,
    pub created_at: DateTime,
    pub category_id: Uuid,
    pub category_name: String,
    pub tags: Option<InlineVec<headless_tag::HeadlessTag>>,
    pub sub_categories: InlineVec<headless_sub_category::HeadlessSubCategory>,
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
            tags: match data.tags {
                Some(tags) => tags.into_inner(),
                None => vec![],
            },
            sub_categories: data.sub_categories.into_inner(), 
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
                    STRING_AGG(t.id || ',' || t.name || ',' || t.color, ';') AS "tags!: Option<InlineVec<headless_tag::HeadlessTag>>",
                    STRING_AGG(sc.id || ',' || sc.name, ';') AS "sub_categories!: InlineVec<HeadlessSubCategory>"
                FROM blogs b
                JOIN 
                    categories c ON b.category_id = c.id
                LEFT JOIN
                    tags_blogs bt ON b.id = bt.blog_id
                LEFT JOIN
                    tags t ON bt.tag_id = t.id
                JOIN
                    sub_categories_blogs sb ON b.id = sb.blog_id
                JOIN
                    sub_categories sc ON sb.sub_category_id = sc.id
                WHERE b.title ILIKE $1
                GROUP BY
                    b.id, c.id, sc.id
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
