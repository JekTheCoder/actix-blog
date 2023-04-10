use std::future::Future;

use serde::Deserialize;
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate;

use crate::app::{Pool, QueryResult};

pub struct Blog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateReq {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}

async fn create(pool: &Pool, req: CreateReq, user_id: Uuid) -> Result<QueryResult, sqlx::Error> {
    query!(
        "INSERT INTO blogs(user_id, title, content) VALUES($1, $2, $3)",
        user_id,
        req.title,
        req.content
    )
    .execute(pool)
    .await
}

impl Blog {
    pub fn create<'a>(
        pool: &'a Pool,
        req: &'a CreateReq,
        user_id: Uuid,
    ) -> impl Future<Output = Result<QueryResult, sqlx::Error>> + 'a {
        query!(
            "INSERT INTO blogs(user_id, title, content) VALUES($1, $2, $3)",
            user_id,
            req.title,
            req.content
        )
        .execute(pool)
    }

    pub fn by_id<'a>(
        pool: &'a Pool,
        id: Uuid,
    ) -> impl Future<Output = Result<Blog, sqlx::Error>> + 'a {
        query_as!(
            Blog,
            "SELECT id, title, content, user_id FROM blogs WHERE id = $1",
            id
        )
        .fetch_one(pool)
    }
}
