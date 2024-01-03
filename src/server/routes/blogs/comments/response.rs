use serde::Serialize;
use uuid::Uuid;

use crate::{shared::models::response::public_account::PublicAccount, domain::comment::CommentJoinUser};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentByBlog {
    pub id: Uuid,
    pub blog_id: Uuid,
    pub content: String,
    pub account: PublicAccount,
    pub has_replies: bool,
}

impl From<CommentJoinUser> for CommentByBlog {
    fn from(comment: CommentJoinUser) -> Self {
        Self {
            id: comment.id,
            blog_id: comment.blog_id,
            content: comment.content,
            has_replies: comment.has_replies,
            account: PublicAccount {
                id: comment.account_id,
                name: comment.account_name,
                username: comment.account_username,
            },
        }
    }
}
