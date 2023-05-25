use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct BlogPreview {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: Option<String>,
}
