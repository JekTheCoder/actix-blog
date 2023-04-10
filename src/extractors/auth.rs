use std::future::ready;

use crate::{
    app::AppState,
    error::http::code::HttpCode,
    models::user::User,
    services::auth::AuthDecoder,
    utils::{future::Future, http::bearer},
};
use actix_web::{error::ErrorUnauthorized, FromRequest, web::Data};
use sqlx::query_as;

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

        let decoder = req.app_data::<Data<AuthDecoder>>().expect("Decoder not found");
        let id = match decoder.decode(token) {
            Ok(decoded) => decoded,
            _ => return Box::pin(ready(Err(ErrorUnauthorized("")))),
        };

        let app = req
            .app_data::<Data<AppState>>()
            .expect("App state not found")
            .clone();

        Box::pin(async move {
            let user = query_as!(User, "SELECT * FROM users WHERE id = $1", id)
                .fetch_one(&app.pool)
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
