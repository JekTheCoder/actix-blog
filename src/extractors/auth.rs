use std::future::ready;

use crate::{
    models::user::User,
    services::auth::AuthDecoder,
    utils::{future::Future, http::bearer}, db::Pool,
};
use actix_web::{error::ErrorUnauthorized, web::Data, FromRequest};

#[derive(Debug)]
pub struct AuthUser {
    pub user: User,
}

impl FromRequest for AuthUser {
    type Error = actix_web::Error;
    type Future = Future<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = bearer(req);
        let token = match token {
            Some(t) => t,
            None => return Box::pin(ready(Err(ErrorUnauthorized("")))),
        };

        let decoder = req
            .app_data::<Data<AuthDecoder>>()
            .expect("Decoder not found");
        let id = match decoder.decode(token) {
            Ok(decoded) => decoded,
            _ => return Box::pin(ready(Err(ErrorUnauthorized("")))),
        };

        let pool = req
            .app_data::<Data<Pool>>()
            .expect("App state not found")
            .clone();

        Box::pin(async move {
            let user = User::complete_by_id(&pool, id)
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

    pub fn get_ref(&self) -> &User {
        &self.user
    }
}
