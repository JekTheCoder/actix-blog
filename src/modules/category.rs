pub use db::{
    get_all_categories, get_all_sub_categories, get_all_tags, get_sub_categories_by_category,
    get_tags_by_category, link_sub_categories, link_tags,
};
pub use models::{Category, SubCategory, Tag};

mod models {
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct Category {
        pub id: uuid::Uuid,
        pub name: String,
    }

    #[derive(Serialize, Debug)]
    pub struct SubCategory {
        pub id: uuid::Uuid,
        pub name: String,
        pub category_id: uuid::Uuid,
    }

    #[derive(Serialize, Debug)]
    pub struct Tag {
        pub id: uuid::Uuid,
        pub name: String,
        pub color: String,
        pub category_id: uuid::Uuid,
    }
}

mod db {
    use crate::modules::{
        category::SubCategory,
        db::{Driver, Pool, QueryResult},
    };
    use sqlx::{query_as, QueryBuilder};

    use crate::modules::category::models::Category;

    use super::Tag;

    pub async fn get_all_categories(pool: &Pool) -> Result<Vec<Category>, sqlx::Error> {
        query_as!(Category, "SELECT * FROM categories")
            .fetch_all(pool)
            .await
    }

    pub async fn link_sub_categories(
        pool: &Pool,
        sub_categories: Vec<uuid::Uuid>,
        blog_id: uuid::Uuid,
    ) -> Result<QueryResult, sqlx::Error> {
        let mut query_builder = QueryBuilder::<'_, Driver>::new(
            "INSERT INTO sub_categories_blogs (blog_id, sub_category_id) ",
        );

        query_builder.push_values(sub_categories, |mut query, sub_category| {
            query.push_bind(blog_id).push_bind(sub_category);
        });

        let query = query_builder.build();
        query.execute(pool).await
    }

    pub async fn get_all_sub_categories(pool: &Pool) -> Result<Vec<SubCategory>, sqlx::Error> {
        query_as!(SubCategory, "SELECT * FROM sub_categories")
            .fetch_all(pool)
            .await
    }

    pub async fn get_sub_categories_by_category(
        pool: &Pool,
        category_id: uuid::Uuid,
    ) -> Result<Vec<SubCategory>, sqlx::Error> {
        query_as!(
            SubCategory,
            "SELECT * FROM sub_categories WHERE category_id = $1",
            category_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn link_tags(
        pool: &Pool,
        tags: Vec<uuid::Uuid>,
        blog_id: uuid::Uuid,
    ) -> Result<QueryResult, sqlx::Error> {
        let mut query_builder =
            QueryBuilder::<'_, Driver>::new("INSERT INTO tags_blogs (blog_id, tag_id) ");

        query_builder.push_values(tags, |mut query, tag| {
            query.push_bind(blog_id).push_bind(tag);
        });

        let query = query_builder.build();
        query.execute(pool).await
    }

    pub async fn get_all_tags(pool: &Pool) -> Result<Vec<Tag>, sqlx::Error> {
        query_as!(Tag, "SELECT * FROM tags").fetch_all(pool).await
    }

    pub async fn get_tags_by_category(
        pool: &Pool,
        category_id: uuid::Uuid,
    ) -> Result<Vec<Tag>, sqlx::Error> {
        query_as!(
            Tag,
            "SELECT * FROM tags WHERE category_id = $1",
            category_id
        )
        .fetch_all(pool)
        .await
    }
}
