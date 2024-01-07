pub use is_admin_middleware::{IsAdminFactory, IsAdminMiddleware};

mod is_admin_middleware {
    use std::future::{ready, Ready};

    use actix_web::dev::{Service, ServiceRequest, Transform};
    use actix_web::error::{ErrorForbidden, ErrorUnauthorized, ErrorInternalServerError};
    use actix_web::{HttpRequest, HttpResponse};

    use actix_web::{
        dev::{forward_ready, ServiceResponse},
        Error,
    };
    use futures_util::future::LocalBoxFuture;

    use crate::domain::user::features::convert_to_admin_id::ConvertToAdminId;
    use crate::domain::user::value_objects::AdminId;
    use crate::server::service::FromRequestSync;

    use super::uncheked_admin_id::{self, get_unchecked_admin_id};

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
            let unchecked_id = match get_unchecked_admin_id(req.request()) {
                Ok(id) => id,
                Err(e) => {
                    let e = match e {
                        uncheked_admin_id::Error::Claims => ErrorUnauthorized(""),
                        uncheked_admin_id::Error::NotAdmin => ErrorForbidden(""),
                    };

                    return Box::pin(ready(Err(e)));
                }
            };

            let Ok(admin_check) = ConvertToAdminId::sync_from_request(req.request()) else {
                return Box::pin(ready(Err(ErrorInternalServerError(""))));
            };

            let next_res = self.service.call(req);

            Box::pin(async move {
                match admin_check.run(unchecked_id).await {
                    Ok(_) => next_res.await,
                    Err(_) => Err(ErrorForbidden("")),
                }
            })
        }
    }
}

pub mod uncheked_admin_id {
    use uuid::Uuid;

    use crate::server::auth::{Claims, Role};

    pub enum Error {
        Claims,
        NotAdmin,
    }

    pub fn get_unchecked_admin_id(req: &actix_web::HttpRequest) -> Result<Uuid, Error> {
        match Claims::from_req(req) {
            Ok(Claims {
                id,
                role: Role::Admin,
                ..
            }) => Ok(id),
            Ok(_) => Err(Error::NotAdmin),
            Err(_) => Err(Error::Claims),
        }
    }
}
