use serde::Serialize;
use uuid::Uuid;

use super::comments::response::CommentByBlog;

#[derive(Serialize)]
pub struct BlogResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub comments: Vec<CommentByBlog>,
}
