use actix_web::web::Data;
use uuid::Uuid;

use crate::{persistence::db::Pool, server::service::sync_service};

sync_service!(GetContent; pool: Data<Pool>);

struct BlogContent {
    content: String,
}

impl GetContent {
    pub async fn run(&self, id: Uuid) -> Result<Option<String>, sqlx::Error> {
        let blog = sqlx::query_as!(BlogContent, "SELECT content FROM blogs WHERE id = $1", id)
            .fetch_optional(self.pool.as_ref()).await?;

        Ok(blog.map(|blog| blog.content))
    }
}
