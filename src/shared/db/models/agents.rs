use serde::Serialize;
use sqlx::query_as;
use uuid::Uuid;

use crate::{error::sqlx::select::SelectErr, shared::db::Pool};

// Common info of an user or an admin
#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub password: String,

    pub name: String,
    pub kind: AgentType,
}

#[derive(sqlx::Type, Clone, Debug)]
#[sqlx(type_name = "account_kind", rename_all = "lowercase")]
pub enum AgentType {
    User,
    Admin,
}

pub async fn by_username(pool: &Pool, username: &str) -> Result<Account, SelectErr> {
    query_as!(
        Account,
        r#"SELECT id, username, password, name, kind AS "kind: _" FROM accounts WHERE username = $1;"#,
        username
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.into())
}

#[derive(Serialize)]
pub struct AgentResponse {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub kind: AgentType,
}

impl From<Account> for AgentResponse {
    fn from(value: Account) -> Self {
        Self {
            id: value.id,
            username: value.username,
            name: value.name,
            kind: value.kind,
        }
    }
}

impl Serialize for AgentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(match self {
            AgentType::User => 0,
            AgentType::Admin => 1,
        })
    }
}
