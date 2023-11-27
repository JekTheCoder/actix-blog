use std::future;

use actix_web::{web::Data, FromRequest};
use uuid::Uuid;

use crate::{
    modules::{
        auth::{Claims, Role},
        db::Pool,
    },
    utils::future::DynFuture,
};

use super::error::AdminError;

pub struct AdminId {
    pub id: Uuid,
}

impl FromRequest for AdminId {
    type Error = AdminError;
    type Future = DynFuture<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let claims = match Claims::from_req(req) {
            Ok(claims) => claims,
            Err(_) => return Box::pin(future::ready(Err(AdminError::Claimns))),
        };

        if claims.role != Role::Admin {
            return Box::pin(future::ready(Err(AdminError::NotAdmin)));
        }

        let pool = req
            .app_data::<Data<Pool>>()
            .expect("Pool not found")
            .clone();

        Box::pin(async move {
            match by_account_id(&pool, claims.id).await {
                Ok(Some(admin_id)) => Ok(admin_id),
                Ok(None) => Err(AdminError::NotFound),
                Err(_) => Err(AdminError::Database),
            }
        })
    }
}

async fn by_account_id<'a>(
    pool: &'a Pool,
    account_id: Uuid,
) -> Result<Option<AdminId>, sqlx::Error> {
    sqlx::query_as!(
        AdminId,
        "SELECT id FROM admins WHERE account_id = $1",
        account_id
    )
    .fetch_optional(pool)
    .await
}
