use serde::Serialize;
use uuid::Uuid;

use super::comments::comment::Comment;

#[derive(Serialize)]
pub struct BlogResponse {
    pub id: Uuid,
    pub admin_id: Uuid,
    pub title: String,
    pub content: String,
    pub comments: Vec<Comment>
}

