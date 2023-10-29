use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder, ResponseError,
};
use serde::{Deserialize, Serialize};

use crate::modules::{
    auth::{AuthEncoder, ClaimsData},
    db::Pool, account,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub user: account::AccountResponse,
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("username or password invalid")]
    BadRequest,
    #[error("verify error")]
    Verify,
    #[error("token error")]
    Token,
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::BadRequest => actix_web::http::StatusCode::BAD_REQUEST,
            Error::Verify => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Error::Token => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[post("/login/")]
pub async fn endpoint(
    pool: Data<Pool>,
    encoder: Data<AuthEncoder>,
    req: Json<Request>,
) -> Result<impl Responder, Error> {
    let Request { username, password } = req.0;

    let found = account::by_username(pool.get_ref(), &username)
        .await
        .map_err(|_| Error::BadRequest)?;

    let verified = bcrypt::verify(password, &found.password).map_err(|_| Error::Verify)?;

    if verified {
        let tokens = encoder
            .generate_tokens(ClaimsData {
                id: found.id,
                role: found.kind.clone().into(),
            })
            .map_err(|_| Error::Token)?;

        let response = Response {
            user: found.into(),
            refresh_token: tokens.refresh_token,
            token: tokens.token,
        };

        Ok(HttpResponse::Ok().json(response))
    } else {
        Err(Error::BadRequest)
    }
}
