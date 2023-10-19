use serde::Serialize;
use uuid::Uuid;

use crate::shared::db::models::comments::Comment;

#[derive(Serialize)]
pub struct BlogResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub comments: Vec<Comment>,
}
