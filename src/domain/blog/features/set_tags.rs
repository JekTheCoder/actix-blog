use actix_web::web::Data;
use sqlx::query;
use uuid::Uuid;

use crate::{
    domain::blog_grouping,
    persistence::db::{Pool, Transaction},
    server::service::sync_service,
};

sync_service!(SetTags; pool: Data<Pool>);

pub async fn set_tags(
    tx: &mut Transaction<'_>,
    blog_id: Uuid,
    tags: Vec<Uuid>,
) -> Result<(), sqlx::Error> {
    query!("DELETE FROM tags_blogs WHERE blog_id = $1", blog_id)
        .execute(&mut *tx)
        .await?;

    create_tags(tx, blog_id, tags).await?;

    Ok(())
}

pub async fn create_tags(
    tx: &mut Transaction<'_>,
    blog_id: Uuid,
    tags: Vec<Uuid>,
) -> Result<(), sqlx::Error> {
    if !tags.is_empty() {
        blog_grouping::link_tags(tx, tags, blog_id).await?;
    }

    Ok(())
}

impl SetTags {
    pub async fn run(&self, blog_id: Uuid, tags: Vec<Uuid>) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        set_tags(&mut tx, blog_id, tags).await?;

        tx.commit().await?;

        Ok(())
    }
}
