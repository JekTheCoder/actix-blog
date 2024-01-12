use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::account::PublicAccount;

#[derive(Deserialize, Validate)]
pub struct CreateComment {
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Serialize)]
pub struct Comment {
    pub id: Uuid,
    pub account_id: Uuid,
    pub blog_id: Uuid,
    pub content: String,
}

pub struct CommentJoinUser {
    pub id: Uuid,
    pub blog_id: Uuid,
    pub content: String,
    pub account_id: Uuid,
    pub account_name: String,
    pub account_username: String,
    pub has_replies: bool,
}

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
