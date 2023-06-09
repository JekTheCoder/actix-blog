use actix_web::{
    post,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse, Responder, ResponseError,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::http::{code::HttpCode, json::JsonResponse},
    models::{login::LoginResponse, user::{User, self}},
    services::auth::{encoder::AuthEncoder, RefreshDecoder},
    traits::catch_http::CatchHttp,
    utils::http::bearer, db::Pool,
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

    let found = User::by_username(pool.get_ref(), &username)
        .await
        .map_err(|_| LoginInvalid)?;

    let verified =
        bcrypt::verify(password, &found.password).map_err(|_| HttpCode::internal_error())?;

    match verified {
        false => Err(LoginInvalid.into()),
        true => {
            let tokens = encoder
                .generate_tokens(found.id)
                .map_err(|_| HttpCode::internal_error())?;

            let response = LoginResponse {
                user: found.into(),
                refresh_token: tokens.refresh_token,
                token: tokens.token,
            };

            Ok(HttpResponse::Ok().json(response))
        }
    }
}

#[post("/register/")]
async fn register(
    pool: Data<Pool>,
    encoder: Data<AuthEncoder>,
    req: Json<user::CreateReq>,
) -> actix_web::Result<impl Responder> {
    req.validate()
        .map_err(|reason| JsonResponse::body(reason))?;

    let id = User::create(pool.get_ref(), &req.0).await.catch_http()?;
    let inner = req.into_inner();

    let user_res = user::Response {
        id,
        name: inner.name,
        username: inner.username,
    };
    let tokens = encoder
        .generate_tokens(id)
        .map_err(|_| HttpCode::internal_error())?;

    Ok(HttpResponse::Created().json(LoginResponse::new(user_res, tokens)))
}

#[derive(Debug, Deserialize)]
pub struct RefreshReq {
    pub refresh_token: String,
}   

#[get("/refresh/")]
async fn refresh(
    req: Json<RefreshReq>,
    decoder: Data<RefreshDecoder>,
    encoder: Data<AuthEncoder>,
    refresh: Json<RefreshReq>,
) -> actix_web::Result<impl Responder> {
    let refresh_token = bearer(&req).ok_or_else(|| HttpCode::unauthorized())?;
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
