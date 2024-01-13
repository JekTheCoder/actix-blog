use actix_web::{post, web::Data, HttpResponse, Responder, ResponseError};
use serde::Serialize;

use crate::domain::user::value_objects::UsernameBuf;
use crate::server::auth::{AuthEncoder, ClaimsData};
use crate::{
    domain::{account, user::features::login},
    server::shared::{domain_validation::domain_valid, query::DomainJson},
};

domain_valid!(pub struct Request {
    username: UsernameBuf,
    password: String,
}; UncheckedRequest);

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
    encoder: Data<AuthEncoder>,
    login: login::Login,
    req: DomainJson<Request>,
) -> Result<impl Responder, Error> {
    let Request { username, password } = req.into_inner();

    let account = match login.run(&username, &password).await {
        Ok(account) => account,
        Err(login::Error::NotFound) => return Err(Error::BadRequest),
        Err(login::Error::Password) => return Err(Error::Verify),
        Err(login::Error::Database) => return Err(Error::BadRequest),
    };

    let tokens = encoder
        .generate_tokens(ClaimsData {
            id: account.id,
            role: account.kind.clone(),
        })
        .map_err(|_| Error::Token)?;

    let response = Response {
        user: account.into(),
        refresh_token: tokens.refresh_token,
        token: tokens.token,
    };

    Ok(HttpResponse::Ok().json(response))
}
