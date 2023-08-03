use crate::{
    error::sqlx::{insert::InsertErr, select::SelectErr},
    shared::{db::Pool, models::insert_return::IdMaybe},
};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub email: Option<String>,
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
    pub id: Uuid,
}

impl From<User> for Response {
    fn from(value: User) -> Self {
        Self { id: value.id }
    }
}

pub async fn create(pool: &Pool, req: &CreateReq) -> Result<Uuid, InsertErr> {
    let CreateReq {
        username,
        name,
        email,
        password,
    } = req;
    let password = bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|_| InsertErr::Unknown)?;

    // selecting from a function returns a nullable value, even if we know that it is not null. 
    // We need to handle this.
    let result = query_as!(
        IdMaybe,
        "SELECT insert_user($1, $2, $3, $4) AS id",
        username,
        password,
        name,
        email.as_ref(),
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(IdMaybe { id }) => id.ok_or_else(|| InsertErr::Unknown),
        Err(e) => Err(e.into()),
    }
}

pub async fn by_id(pool: &Pool, id: Uuid) -> Result<Response, SelectErr> {
    query_as!(Response, "SELECT id FROM users WHERE id = $1", id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
}

pub async fn complete_by_id(pool: &Pool, id: Uuid) -> Result<User, SelectErr> {
    query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
}
