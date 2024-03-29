pub use db::{create, get_many, get_many_by_parent};
pub use models::ReplyJoinAccount;

mod models {
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ReplyJoinAccount {
        pub id: Uuid,
        pub comment_id: Uuid,
        pub parent_id: Option<Uuid>,
        pub content: String,
        pub has_replies: bool,

        pub account_id: Uuid,
        pub account_name: String,
        pub account_username: String,
    }
}

mod db {
    use crate::persistence::db::{entities::IdSelect, Pool, Slice};
    use sqlx::query_as;
    use uuid::Uuid;

    use super::models::ReplyJoinAccount;

    pub async fn get_many_by_parent(
        pool: &Pool,
        comment_id: Uuid,
        parent_id: Uuid,
        Slice { limit, offset }: Slice,
    ) -> Result<Vec<ReplyJoinAccount>, sqlx::Error> {
        query_as!(
        ReplyJoinAccount,
        r#"SELECT 
            ro.id, ro.comment_id, ro.parent_id, ro.content,
            a.id as account_id, a.name as account_name, a.username as account_username, 
            (SELECT COUNT(*) > 0 FROM replies ri WHERE ri.parent_id = ro.id LIMIT 1) as "has_replies!"
            FROM replies ro
            JOIN accounts a on ro.account_id = a.id 
            WHERE comment_id = $1 AND parent_id = $2
            ORDER BY ro.created_at DESC
            LIMIT $3 OFFSET $4"#,
        comment_id,
        parent_id,
        limit,
        offset,
    )
    .fetch_all(pool)
    .await
    }

    pub async fn get_many(
        pool: &Pool,
        comment_id: Uuid,
        Slice { limit, offset }: Slice,
    ) -> Result<Vec<ReplyJoinAccount>, sqlx::Error> {
        query_as!(
        ReplyJoinAccount,
        r#"SELECT 
            ro.id, ro.comment_id, ro.parent_id, ro.content,
            a.id as account_id, a.name as account_name, a.username as account_username, 
            (SELECT COUNT(*) > 0 FROM replies ri WHERE ri.parent_id = ro.id LIMIT 1) as "has_replies!"
            FROM replies ro
            JOIN accounts a on ro.account_id = a.id 
            WHERE comment_id = $1 AND parent_id IS NULL 
            ORDER BY ro.created_at DESC
            LIMIT $2 OFFSET $3"#,
            comment_id,
            limit,
            offset,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &Pool,
        content: &str,
        account_id: Uuid,
        comment_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<IdSelect, sqlx::Error> {
        query_as!(
            IdSelect,
            "INSERT INTO replies (content, account_id, comment_id, parent_id) \
            VALUES ($1, $2, $3, $4) RETURNING id",
            content,
            account_id,
            comment_id,
            parent_id,
        )
        .fetch_one(pool)
        .await
    }
}
