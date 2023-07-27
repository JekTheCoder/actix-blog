use std::future::Future;

use futures_util::TryFutureExt;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate;

use super::blog_preview::BlogPreview;
use crate::{
    db::{Pool, QueryResult},
    error::sqlx::{insert::InsertErr, select::SelectErr},
    models::select_slice::SelectSlice,
};

#[derive(Serialize)]
pub struct Blog {
    pub id: Uuid,
    pub admin_id: Uuid,
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
        admin_id: Uuid,
    ) -> impl Future<Output = Result<QueryResult, InsertErr>> + 'a {
        Box::pin(async move {
            query!(
                "INSERT INTO blogs(admin_id, title, content) VALUES($1, $2, $3)",
                admin_id,
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
                "SELECT id, title, content, admin_id FROM blogs WHERE id = $1",
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
    ) -> impl Future<Output = Result<Vec<BlogPreview>, SelectErr>> + 'a {
        let SelectSlice { limit, offset } = slice;

        query_as!(
                BlogPreview,
                "SELECT id, title, SUBSTRING(content, 0, 200) as content, admin_id FROM blogs \
                LIMIT $1 OFFSET $2",
                limit,
                offset
            )
            .fetch_all(pool)
            .map_err(|e| e.into())
    }
}
