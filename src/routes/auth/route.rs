use std::ops::Deref;

use actix_web::{
    post,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse, Responder, ResponseError,
};
use serde::Deserialize;
use validator::Validate;

use super::response::LoginResponse;
use crate::{
    error::http::{code::HttpCode, json::JsonResponse},
    services::auth::{claims::InnerClaims, encoder::AuthEncoder, RefreshDecoder},
    shared::db::{models::agents, Pool},
    shared::{db::models::users, extractors::valid_json::ValidJson},
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

    let found = agents::by_username(pool.get_ref(), &username)
        .await
        .map_err(|_| LoginInvalid)?;

    let verified =
        bcrypt::verify(password, &found.password).map_err(|_| HttpCode::internal_error())?;

    if verified {
        let tokens = encoder
            .generate_tokens(InnerClaims::user_claims(found.id))
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
    req: ValidJson<users::CreateReq>,
) -> actix_web::Result<impl Responder> {
    // req.validate()
    //     .map_err(|reason| JsonResponse::body(reason))?;

    let id = users::create(pool.get_ref(), req.as_ref()).await.catch_http()?;
    let users::CreateReq { name, username, .. } = req.into_inner();
    let agent_response = agents::AgentResponse {
        id,
        r#type: agents::AgentType::User,
        name,
        username,
    };

    let tokens = encoder
        .generate_tokens(InnerClaims::user_claims(id))
        .map_err(|_| HttpCode::internal_error())?;

    Ok(HttpResponse::Created().json(LoginResponse::new(agent_response, tokens)))
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
    let claims = decoder
        .decode(&req.refresh_token)
        .map_err(|_| HttpCode::unauthorized())?;

    let now: usize = match chrono::Utc::now().timestamp_millis().try_into() {
        Ok(time) => time,
        Err(_) => return Err(HttpCode::unauthorized().into()),
    };
    if now > claims.exp {
        return Err(HttpCode::unauthorized().into());
    }

    let tokens = encoder
        .generate_tokens(claims.inner())
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
