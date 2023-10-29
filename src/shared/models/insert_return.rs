use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct IdSelect {
    pub id: Uuid,
}
