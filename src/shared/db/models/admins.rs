use sqlx::query_as;
use uuid::Uuid;

use crate::{error::sqlx::select::SelectErr, shared::db::Pool};

pub struct AdminMin {
    pub id: Uuid,
}

pub async fn by_agent_id(agent_id: Uuid, pool: &Pool) -> Result<AdminMin, SelectErr> {
    query_as!(
        AdminMin,
        "SELECT id FROM admins WHERE agent_id = $1",
        agent_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.into())
}
