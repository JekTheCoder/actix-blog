use actix_web::web::Data;
use sqlx::query_as;
use tokio::join;
use uuid::Uuid;

use crate::{
    domain::comment::{self, models::CommentByBlog},
    persistence::db::{DateTime, Executor, Pool, Slice},
    server::service::sync_service,
};

sync_service!(GetById; pool: Data<Pool>);

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogById {
    pub id: Uuid,
    pub title: String,
    pub html: String,
    pub created_at: DateTime,
    pub comments: Vec<CommentByBlog>,
}

struct RawBlogById {
    pub id: Uuid,
    pub title: String,
    pub html: String,
    pub created_at: DateTime,
}

impl GetById {
    pub async fn run(&self, id: Uuid) -> Result<Option<BlogById>, sqlx::Error> {
        let (blog, comments) = join!(
            get_by_id(self.pool.get_ref(), id),
            comment::by_blog(
                self.pool.get_ref(),
                id,
                Slice {
                    limit: 20,
                    offset: 0
                }
            ),
        );

        let Some(blog) = blog? else {
            return Ok(None);
        };

        let comments = comments.unwrap_or_else(|_| vec![]);

        let blog = BlogById {
            id: blog.id,
            title: blog.title,
            html: blog.html,
            created_at: blog.created_at,
            comments: comments.into_iter().map(Into::into).collect(),
        };

        Ok(Some(blog))
    }
}

async fn get_by_id(pool: impl Executor<'_>, id: Uuid) -> Result<Option<RawBlogById>, sqlx::Error> {
    query_as!(
        RawBlogById,
        "SELECT id, title, html, created_at FROM blogs WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
}
