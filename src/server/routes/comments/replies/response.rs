use serde::Serialize;
use uuid::Uuid;

use crate::domain::{account::PublicAccount, reply::ReplyJoinAccount};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyByComment {
    pub id: Uuid,
    pub blog_id: Uuid,
    pub content: String,
    pub account: PublicAccount,
    pub has_replies: bool,
}

impl From<ReplyJoinAccount> for ReplyByComment {
    fn from(reply: ReplyJoinAccount) -> Self {
        Self {
            id: reply.id,
            blog_id: reply.comment_id,
            content: reply.content,
            has_replies: reply.has_replies,
            account: PublicAccount {
                id: reply.account_id,
                name: reply.account_name,
                username: reply.account_username,
            },
        }
    }
}
