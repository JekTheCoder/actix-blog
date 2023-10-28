use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct PublicAccount {
    pub id: Uuid,
    pub name: String,
    pub username: String,
}
