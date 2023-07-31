use actix_web::{
    post,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse, Responder, ResponseError,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::http::{code::HttpCode, json::JsonResponse},
    models::login::LoginResponse,
    services::auth::{encoder::AuthEncoder, RefreshDecoder},
    shared::db::models::users,
    shared::db::Pool,
    traits::catch_http::CatchHttp,
};

#[derive(Clone, Debug, Deserialize)]
struct LoginReq {
    username: String,
    password: String,
}

#[derive(Debug, thiserror::Error)]
#[error("username or password invalid")]
struct LoginInvalid;

impl ResponseError for LoginInvalid {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }
}

#[post("/login/")]
async fn login(
    pool: Data<Pool>,
    encoder: Data<AuthEncoder>,
    req: Json<LoginReq>,
) -> actix_web::Result<impl Responder> {
    let LoginReq { username, password } = req.0;

    let found = users::by_username(pool.get_ref(), &username)
        .await
        .map_err(|_| LoginInvalid)?;

    let verified =
        bcrypt::verify(password, &found.password).map_err(|_| HttpCode::internal_error())?;

    if verified {
        let tokens = encoder
            .generate_tokens(found.id)
            .map_err(|_| HttpCode::internal_error())?;

        let response = LoginResponse {
            user: found.into(),
            refresh_token: tokens.refresh_token,
            token: tokens.token,
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        Err(LoginInvalid.into())
    }
}

#[post("/register/")]
async fn register(
    pool: Data<Pool>,
    encoder: Data<AuthEncoder>,
    req: Json<users::CreateReq>,
) -> actix_web::Result<impl Responder> {
    req.validate()
        .map_err(|reason| JsonResponse::body(reason))?;

    let id = users::create(pool.get_ref(), &req.0).await.catch_http()?;

    let users::CreateReq { name, username, .. } = req.into_inner();
    let user_res = users::Response { id, name, username };

    let tokens = encoder
        .generate_tokens(id)
        .map_err(|_| HttpCode::internal_error())?;

    Ok(HttpResponse::Created().json(LoginResponse::new(user_res, tokens)))
}

#[derive(Debug, Deserialize)]
pub struct RefreshReq {
    pub refresh_token: String,
}

#[post("/refresh/")]
async fn refresh(
    req: Json<RefreshReq>,
    decoder: Data<RefreshDecoder>,
    encoder: Data<AuthEncoder>,
) -> actix_web::Result<impl Responder> {
    let id = decoder
        .decode(&req.refresh_token)
        .map_err(|_| HttpCode::unauthorized())?;

    let tokens = encoder
        .generate_tokens(id)
        .map_err(|_| HttpCode::internal_error())?;

    Ok(HttpResponse::Ok().json(tokens))
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(login)
            .service(register)
            .service(refresh),
    );
}
