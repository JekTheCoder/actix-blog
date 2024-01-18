pub use db::{by_blog, create};

pub mod models;

mod db {
    use sqlx::query_as;
    use uuid::Uuid;

    use crate::persistence::db::{entities::IdSelect, Pool, Slice};

    use super::models::{CommentJoinUser, CreateComment};

    pub async fn by_blog(
        pool: &Pool,
        blog_id: Uuid,
        Slice { limit, offset }: Slice,
    ) -> Result<Vec<CommentJoinUser>, sqlx::Error> {
        query_as!(
        CommentJoinUser,
        r#"SELECT 
            c.id, c.blog_id, c.content, 
            a.id as account_id, a.name as account_name, a.username as account_username, 
            (SELECT COUNT(*) > 0 FROM replies r WHERE r.comment_id = c.id AND r.parent_id IS NULL LIMIT 1) as "has_replies!"
            FROM comments c 
            JOIN accounts a on c.account_id = a.id 
            WHERE blog_id = $1 
            ORDER BY c.created_at DESC
            LIMIT $2 OFFSET $3"#,
        blog_id,
        limit,
        offset
        )
        .fetch_all(pool)
        .await
    }

    pub async fn create<'a>(
        pool: &'a Pool,
        req: &'a CreateComment,
        agent_id: Uuid,
        blog_id: Uuid,
    ) -> Result<IdSelect, sqlx::Error> {
        query_as!(
            IdSelect,
            "INSERT INTO comments (account_id, blog_id, content) VALUES ($1, $2, $3) RETURNING id",
            agent_id,
            blog_id,
            req.content
        )
        .fetch_one(pool)
        .await
    }
}
