use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder, ResponseError,
};
use serde::Deserialize;

use crate::modules::auth::{AuthEncoder, RefreshDecoder};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("invalid claims")]
    ClaimsDecode,
    #[error("internal error")]
    HostTime,
    #[error("claims expired")]
    Expired,
    #[error("internal error")]
    Tokens,
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::Expired => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub refresh_token: String,
}

#[post("/refresh/")]
async fn endpoint(
    req: Json<Request>,
    decoder: Data<RefreshDecoder>,
    encoder: Data<AuthEncoder>,
) -> Result<impl Responder, Error> {
    let claims = decoder
        .decode(&req.refresh_token)
        .map_err(|_| Error::ClaimsDecode)?;

    let now: usize = match chrono::Utc::now().timestamp().try_into() {
        Ok(time) => time,
        Err(_) => return Err(Error::HostTime),
    };

    if now > claims.exp {
        return Err(Error::Expired);
    }

    let tokens = encoder
        .generate_tokens(claims.inner())
        .map_err(|_| Error::Tokens)?;

    Ok(HttpResponse::Ok().json(tokens))
}
