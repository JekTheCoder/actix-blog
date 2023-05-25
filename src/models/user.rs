use std::future::Future;

use crate::{
    db::Pool,
    error::sqlx::{insert::InsertErr, select::SelectErr},
    models::insert_return::IdReturn,
};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub name: String,
    pub email: Option<String>,
    pub id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct CreateReq {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email(message = "email not valid"))]
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub username: String,
    pub name: String,
    pub id: Uuid,
}

impl From<User> for Response {
    fn from(value: User) -> Self {
        Self {
            username: value.username,
            name: value.name,
            id: value.id,
        }
    }
}

impl User {
    pub fn by_username<'a>(
        pool: &'a Pool,
        username: &'a str,
    ) -> impl Future<Output = Result<User, SelectErr>> + 'a {
        Box::pin(async move {
            query_as!(User, "SELECT * FROM users WHERE username = $1", username)
                .fetch_one(pool)
                .await
                .map_err(|e| e.into())
        })
    }

    pub fn create<'a>(
        pool: &'a Pool,
        req: &'a CreateReq,
    ) -> impl Future<Output = Result<Uuid, InsertErr>> + 'a {
        Box::pin(async move {
            let CreateReq {
                username,
                name,
                email,
                password,
            } = req;
            let password =
                bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|_| InsertErr::Unknown)?;

            query_as!(
                IdReturn,
                "INSERT INTO users(username, password, name, email) VALUES($1, $2, $3, $4) RETURNING id",
                username,
                password,
                name,
                email.as_ref(),
            )
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
            .map(|id| id.id)
        })
    }

    pub fn by_id<'a>(
        pool: &'a Pool,
        id: Uuid,
    ) -> impl Future<Output = Result<Response, SelectErr>> + 'a {
        Box::pin(async move {
            query_as!(
                Response,
                "SELECT username, name, id FROM users WHERE id = $1",
                id
            )
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
        })
    }

    pub fn complete_by_id<'a>(
        pool: &'a Pool,
        id: Uuid,
    ) -> impl Future<Output = Result<User, SelectErr>> + 'a {
        Box::pin(async move {
            query_as!(User, "SELECT * FROM users WHERE id = $1", id)
                .fetch_one(pool)
                .await
                .map_err(|e| e.into())
        })
    }
}
