use actix_web::web::Data;

use crate::{
    domain::blog_grouping::sub_category::SubCategory, persistence::db::Pool,
    server::service::sync_service,
};

sync_service!(GetSubCategorysByBlog; pool: Data<Pool>);

impl GetSubCategorysByBlog {
    pub async fn run(&self, blog_id: uuid::Uuid) -> Result<Vec<SubCategory>, sqlx::Error> {
        sqlx::query_as!(
            SubCategory,
            "SELECT sc.* FROM sub_categories sc JOIN sub_categories_blogs scb ON scb.sub_category_id = sc.id WHERE scb.blog_id = $1",
            blog_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }
}
