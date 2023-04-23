use actix_web::{
    post,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse, Responder, ResponseError,
};
use serde::Deserialize;
use sqlx::query_as;
use validator::Validate;

use crate::{
    app::AppData,
    error::http::{code::HttpCode, json::JsonResponse},
    models::{
        insert_return::IdReturn,
        login::LoginResponse,
        user::{self, CreateReq},
    },
    services::auth::{encoder::AuthEncoder, RefreshDecoder},
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
    app: AppData,
    encoder: Data<AuthEncoder>,
    req: Json<LoginReq>,
) -> actix_web::Result<impl Responder> {
    let LoginReq { username, password } = req.0;

    let found = query_as!(
        user::User,
        "SELECT * FROM users WHERE username = $1",
        username
    )
    .fetch_one(&app.pool)
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
    app: AppData,
    encoder: Data<AuthEncoder>,
    req: Json<CreateReq>,
) -> actix_web::Result<impl Responder> {
    req.validate()
        .map_err(|reason| JsonResponse::body(reason))?;

    let CreateReq {
        username,
        password,
        name,
        email,
    } = req.0;

    let password =
        bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|_| HttpCode::internal_error())?;

    let id = query_as!(
        IdReturn,
        "INSERT INTO users(username, password, name, email) VALUES($1, $2, $3, $4) RETURNING id",
        username,
        password,
        name,
        email
    )
    .fetch_one(&app.pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(_) => HttpCode::conflict(),
        _ => HttpCode::internal_error(),
    })?
    .id;

    let user_res = user::Response { id, name, username };
    let tokens = encoder
        .generate_tokens(id)
        .map_err(|_| HttpCode::internal_error())?;

    Ok(HttpResponse::Created().json(LoginResponse::new(user_res, tokens)))
}

#[derive(Deserialize, Debug)]
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
