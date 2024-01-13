use actix_web::web::Data;

use crate::{
    domain::blog_grouping::category::Category, persistence::db::Pool, server::service::sync_service,
};

sync_service!(GetOneCategory; pool: Data<Pool>);

impl GetOneCategory {
    pub async fn run(&self, category_id: uuid::Uuid) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as!(
            Category,
            "SELECT * FROM categories WHERE id = $1",
            category_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
