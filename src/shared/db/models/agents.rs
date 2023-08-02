use serde::Serialize;
use sqlx::query_as;
use uuid::Uuid;

use crate::{error::sqlx::select::SelectErr, shared::db::Pool};

// Common info of an user or an admin
pub struct Agent {
    pub id: Uuid,
    pub username: String,
    pub password: String,

    pub name: String,
    pub r#type: AgentType,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "agent_kind", rename_all = "lowercase")]
#[derive(Serialize)]
pub enum AgentType {
    User,
    Admin,
}

pub async fn by_username(pool: &Pool, username: &str) -> Result<Agent, SelectErr> {
    query_as!(
        Agent,
        r#"SELECT id, username, password, name, type AS "type: _" FROM agents WHERE username = $1;"#,
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
    pub r#type: AgentType,
}

impl From<Agent> for AgentResponse {
    fn from(value: Agent) -> Self {
        Self {
            id: value.id,
            username: value.username,
            name: value.name,
            r#type: value.r#type,
        }
    }
}