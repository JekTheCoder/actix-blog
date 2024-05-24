use actix_web::web::Data;
use sqlx::query_as;
use tokio::join;
use uuid::Uuid;

use crate::{
    domain::{
        blog_grouping::{
            category, get_one_category, get_sub_categories_by_blog, get_tags_by_blog, sub_category,
            tag,
        },
        comment::{self, models::CommentByBlog},
    },
    persistence::db::{DateTime, Executor, Pool, Slice},
    server::service::sync_service,
};

sync_service!(GetById;
    pool: Data<Pool>,
    get_category: get_one_category::GetOneCategory,
    get_tags: get_tags_by_blog::GetTagsByBlog,
    get_sub_categories: get_sub_categories_by_blog::GetSubCategorysByBlog
);

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogById {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub preview: String,
    pub created_at: DateTime,
    pub comments: Vec<CommentByBlog>,
    pub category: category::Category,
    pub tags: Vec<tag::Tag>,
    pub sub_categories: Vec<sub_category::SubCategory>,
}

struct RawBlogById {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub preview: String,
    pub created_at: DateTime,
    pub category_id: Uuid,
}

impl GetById {
    pub async fn run(&self, id: Uuid) -> Result<Option<BlogById>, sqlx::Error> {
        let blog = get_by_id(self.pool.get_ref(), id);
        let comments = comment::by_blog(
            self.pool.get_ref(),
            id,
            Slice {
                limit: 20,
                offset: 0,
            },
        );
        let tags = self.get_tags.run(id);
        let sub_categories = self.get_sub_categories.run(id);

        let (blog, comments, tags, sub_categories) = join!(blog, comments, tags, sub_categories);

        let Some(blog) = blog? else {
            return Ok(None);
        };

        let tags = tags?;
        let sub_categories = sub_categories?;

        let comments = comments.unwrap_or_else(|_| vec![]);

        let Some(category) = self.get_category.run(blog.category_id).await? else {
            return Ok(None);
        };

        let blog = BlogById {
            id: blog.id,
            title: blog.title,
            content: blog.content,
            preview: blog.preview,
            created_at: blog.created_at,
            comments: comments.into_iter().map(Into::into).collect(),
            category,
            tags,
            sub_categories,
        };

        Ok(Some(blog))
    }
}

async fn get_by_id(pool: impl Executor<'_>, id: Uuid) -> Result<Option<RawBlogById>, sqlx::Error> {
    query_as!(
        RawBlogById,
        "SELECT id, title, html as content, preview, category_id, created_at FROM blogs WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
}
