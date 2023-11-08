use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Blog {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub title: String,
    pub content: String,
    pub html: String,
}

pub struct BlogById {
    pub id: Uuid,
    pub title: String,
    pub html: String,
}

#[derive(Serialize)]
pub struct BlogPreview {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub title: String,
    pub html: Option<String>,
}