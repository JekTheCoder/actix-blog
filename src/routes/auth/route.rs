use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder, ResponseError,
};
use serde::Deserialize;

use crate::{
    error::http::code::HttpCode,
    modules::auth::{AuthEncoder, RefreshDecoder},
};

#[derive(Debug, thiserror::Error)]
#[error("username or password invalid")]
struct LoginInvalid;

impl ResponseError for LoginInvalid {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }
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
