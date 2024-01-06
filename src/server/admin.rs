use std::future::{ready, Ready};

use actix_web::dev::{Service, ServiceRequest, Transform};

use actix_web::{
    dev::{forward_ready, ServiceResponse},
    Error,
};
use futures_util::future::LocalBoxFuture;

use crate::domain::user::value_objects::AdminId;

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
        let admin_check = AdminId::from_req(req.request());
        let next_res = self.service.call(req);

        Box::pin(async move {
            match admin_check.await {
                Ok(_) => next_res.await,
                Err(e) => Err(e.into()),
            }
        })
    }
}
