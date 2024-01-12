pub use db::{
    create_category, create_subcategory, create_tag, delete_category, delete_subcategory,
    delete_tag, get_all_categories, get_all_sub_categories, get_sub_categories_by_category,
    get_tags_by_category, link_sub_categories, link_tags,
};
pub use models::{SubCategory, Tag};

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
    use crate::{
        domain::category::SubCategory,
        persistence::db::{entities::IdSelect, Driver, Executor, Pool, QueryResult},
    };
    use sqlx::{query, query_as, QueryBuilder};

    use crate::domain::category::models::Category;

    use super::Tag;

    pub async fn get_all_categories(pool: &Pool) -> Result<Vec<Category>, sqlx::Error> {
        query_as!(Category, "SELECT * FROM categories")
            .fetch_all(pool)
            .await
    }

    pub async fn create_category(pool: &Pool, name: &str) -> Result<IdSelect, sqlx::Error> {
        query_as!(
            IdSelect,
            "INSERT INTO categories (name) VALUES ($1) RETURNING id",
            name
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete_category(pool: &Pool, id: uuid::Uuid) -> Result<QueryResult, sqlx::Error> {
        query!("DELETE FROM categories WHERE id = $1", id)
            .execute(pool)
            .await
    }

    pub async fn create_subcategory(
        pool: &Pool,
        name: &str,
        category_id: uuid::Uuid,
    ) -> Result<IdSelect, sqlx::Error> {
        query_as!(
            IdSelect,
            "INSERT INTO sub_categories (name, category_id) VALUES ($1, $2) RETURNING id",
            name,
            category_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete_subcategory(
        pool: &Pool,
        id: uuid::Uuid,
    ) -> Result<QueryResult, sqlx::Error> {
        query!("DELETE FROM sub_categories WHERE id = $1", id)
            .execute(pool)
            .await
    }

    pub async fn create_tag(
        pool: &Pool,
        category_id: uuid::Uuid,
        name: &str,
        color: &str,
    ) -> Result<IdSelect, sqlx::Error> {
        query_as!(
            IdSelect,
            "INSERT INTO tags (name, category_id, color) VALUES ($1, $2, $3) RETURNING id",
            name,
            category_id,
            color
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete_tag(pool: &Pool, id: uuid::Uuid) -> Result<QueryResult, sqlx::Error> {
        query!("DELETE FROM tags WHERE id = $1", id)
            .execute(pool)
            .await
    }

    pub async fn link_sub_categories(
        pool: impl Executor<'_>,
        sub_categories: &[uuid::Uuid],
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
        pool: impl Executor<'_>,
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
