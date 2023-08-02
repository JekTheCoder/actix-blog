use std::future;

use crate::{
    services::auth::AuthDecoder,
    shared::db::{
        models::users::{self, User},
        Pool,
    },
    utils::{future::DynFuture, http::bearer},
};
use actix_web::{error::ErrorUnauthorized, web::Data, FromRequest};

#[derive(Debug)]
pub struct AuthUser {
    pub user: User,
}

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = DynFuture<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = bearer(req);
        let token = match token {
            Some(t) => t,
            None => return Box::pin(future::ready(Err(ErrorUnauthorized("")))),
        };

        let decoder = req
            .app_data::<Data<AuthDecoder>>()
            .expect("Decoder not found");
        let claims = match decoder.decode(token) {
            Ok(decoded) => decoded,
            _ => return Box::pin(future::ready(Err(ErrorUnauthorized("")))),
        };

        let pool = req
            .app_data::<Data<Pool>>()
            .expect("App state not found")
            .clone();

        Box::pin(async move {
            let user = users::complete_by_id(&pool, claims.id)
                .await
                .map_err(|_| ErrorUnauthorized(""))?;

            Ok(AuthUser { user })
        })
    }
}

impl AuthUser {
    pub fn into_inner(self) -> User {
        self.user
    }
}
