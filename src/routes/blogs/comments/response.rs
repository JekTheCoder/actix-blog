use serde::Serialize;
use uuid::Uuid;

use crate::shared::db::models::comments::CommentJoinUser;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentByBlog {
    pub id: Uuid,
    pub blog_id: Uuid,
    pub content: String,
    pub account: AccountByComment,
}

#[derive(Serialize)]
pub struct AccountByComment {
    pub id: Uuid,
    pub name: String,
    pub username: String,
}

impl From<CommentJoinUser> for CommentByBlog {
    fn from(comment: CommentJoinUser) -> Self {
        Self {
            id: comment.id,
            blog_id: comment.blog_id,
            content: comment.content,
            account: AccountByComment {
                id: comment.account_id,
                name: comment.account_name,
                username: comment.account_username,
            },
        }
    }
}
