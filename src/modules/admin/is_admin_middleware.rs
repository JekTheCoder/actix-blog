use std::future::{self, ready, Ready};

use actix_web::dev::{Service, ServiceRequest, Transform};

use actix_web::web::Data;
use actix_web::{
    dev::{forward_ready, ServiceResponse},
    Error,
};
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

use crate::modules::auth::{Claims, Role};
use crate::modules::db::Pool;

use super::error::AdminError;

pub struct IsAdminFactory;

impl<S, B> Transform<S, ServiceRequest> for IsAdminFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = IsAdminMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let middleware = IsAdminMiddleware { service };
        ready(Ok(middleware))
    }
}

pub struct IsAdminMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for IsAdminMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let claims = match Claims::from_req(req.request()) {
            Ok(claims) => claims,
            Err(_) => return Box::pin(ready(Err(AdminError::Claimns.into()))),
        };

        if claims.role != Role::Admin {
            return Box::pin(future::ready(Err(AdminError::NotAdmin.into())));
        }

        let pool = req
            .app_data::<Data<Pool>>()
            .expect("Pool not found")
            .clone();

        let next_res = self.service.call(req);

        Box::pin(async move {
            match is_admin(&pool, claims.id).await {
                Ok(true) => next_res.await,
                Ok(false) => Err(AdminError::NotAdmin.into()),
                Err(_) => Err(AdminError::Database.into()),
            }
        })
    }
}

async fn is_admin(pool: &Pool, account_id: Uuid) -> Result<bool, sqlx::Error> {
    sqlx::query!(
        "SELECT COUNT(id) as count FROM admins WHERE account_id = $1",
        account_id
    )
    .fetch_optional(pool)
    .await
    .map(|data| data.is_some())
}
