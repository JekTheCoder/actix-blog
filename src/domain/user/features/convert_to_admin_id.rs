use actix_web::web::Data;
use sqlx::query;
use uuid::Uuid;

use crate::{
    domain::user::admin_id::AdminId, persistence::db::Pool, server::service::sync_service,
};

sync_service!(ConvertToAdminId; pool: Data<Pool>);

impl ConvertToAdminId {
    pub async fn run(&self, id: Uuid) -> Result<AdminId, sqlx::Error> {
        let _ = query!("SELECT id FROM admins WHERE id = $1", id)
            .fetch_one(self.pool.as_ref())
            .await?;

        Ok(AdminId::unchecked_new(id))
    }
}
