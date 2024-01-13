use actix_web::web::Data;

use crate::{
    domain::blog_grouping::tag::Tag, persistence::db::Pool, server::service::sync_service,
};

sync_service!(GetTagsByBlog; pool: Data<Pool>);

impl GetTagsByBlog {
    pub async fn run(&self, blog_id: uuid::Uuid) -> Result<Vec<Tag>, sqlx::Error> {
        sqlx::query_as!(
            Tag,
            "SELECT t.* FROM tags t JOIN tags_blogs tb ON tb.tag_id = t.id WHERE tb.blog_id = $1",
            blog_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }
}
