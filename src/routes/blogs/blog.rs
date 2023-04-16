use std::future::Future;

use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::{Pool, QueryResult},
    error::sqlx::{insert::InsertErr, select::SelectErr}, models::select_slice::SelectSlice,
};

#[derive(Serialize)]
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

impl Blog {
    pub fn create<'a>(
        pool: &'a Pool,
        req: &'a CreateReq,
        user_id: Uuid,
    ) -> impl Future<Output = Result<QueryResult, InsertErr>> + 'a {
        Box::pin(async move {
            query!(
                "INSERT INTO blogs(user_id, title, content) VALUES($1, $2, $3)",
                user_id,
                req.title,
                req.content
            )
            .execute(pool)
            .await
            .map_err(|e| e.into())
        })
    }

    pub fn by_id<'a>(
        pool: &'a Pool,
        id: Uuid,
    ) -> impl Future<Output = Result<Blog, SelectErr>> + 'a {
        Box::pin(async move {
            query_as!(
                Blog,
                "SELECT id, title, content, user_id FROM blogs WHERE id = $1",
                id
            )
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
        })
    }

    pub fn get_all<'a>(
        pool: &'a Pool,
        slice: SelectSlice,
    ) -> impl Future<Output = Result<Vec<Blog>, SelectErr>> + 'a {
        let SelectSlice { limit, offset } = slice;

        Box::pin(async move {
            query_as!(
                Blog,
                "SELECT id, title, content, user_id FROM blogs LIMIT $1 OFFSET $2",
                limit,
                offset
            )
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
        })
    }
}
