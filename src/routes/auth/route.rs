use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder, ResponseError,
};
use serde::Deserialize;

use super::response::LoginResponse;
use crate::modules::user;
use crate::{
    error::http::code::HttpCode,
    modules::{
        auth::{AuthEncoder, ClaimsData, RefreshDecoder, Role},
        db::Pool,
    },
    shared::db::models::agents,
    shared::extractors::valid_json::ValidJson,
    traits::catch_http::CatchHttp,
};

#[derive(Debug, thiserror::Error)]
#[error("username or password invalid")]
struct LoginInvalid;

impl ResponseError for LoginInvalid {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }
}

#[post("/register/")]
async fn register(
    pool: Data<Pool>,
    encoder: Data<AuthEncoder>,
    req: ValidJson<user::CreateRequest>,
) -> actix_web::Result<impl Responder> {
    let mut req = req.into_inner();
    let password = bcrypt::hash(req.password, bcrypt::DEFAULT_COST).unwrap();

    req.password = password;

    let id = user::create(pool.get_ref(), &req).await.catch_http()?;

    let user::CreateRequest { name, username, .. } = req;
    let agent_response = agents::AgentResponse {
        id,
        kind: Role::User,
        name,
        username,
    };

    let tokens = encoder
        .generate_tokens(ClaimsData::user_claims(id))
        .map_err(|_| HttpCode::internal_error())?;

    Ok(HttpResponse::Created().json(LoginResponse::new(agent_response, tokens)))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[post("/refresh/")]
async fn refresh(
    req: Json<RefreshRequest>,
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
