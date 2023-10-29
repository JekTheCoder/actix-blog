pub use db::get_all_categories;
pub use models::{Category, SubCategory, Tag};

mod models {
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct Category {
        pub id: uuid::Uuid,
        pub name: String,
    }

    pub struct SubCategory {
        pub id: uuid::Uuid,
        pub name: String,
        pub category_id: uuid::Uuid,
    }

    pub struct Tag {
        pub id: uuid::Uuid,
        pub name: String,
        pub color: String,
        pub category_id: uuid::Uuid,
    }
}

mod db {
    use crate::modules::db::Pool;
    use sqlx::query_as;

    use crate::modules::category::models::Category;

    pub async fn get_all_categories(pool: &Pool) -> Result<Vec<Category>, sqlx::Error> {
        query_as!(Category, "SELECT * FROM categories")
            .fetch_all(pool)
            .await
    }
}
