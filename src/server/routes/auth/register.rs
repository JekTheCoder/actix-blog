use actix_web::{http::StatusCode, post, web::Data, HttpResponse, Responder, ResponseError};

use crate::{
    domain::{
        account,
        user::{
            features::register::{self, Register},
            value_objects::{EmailBuf, UsernameBuf},
        },
    },
    server::{
        auth::{AuthEncoder, ClaimsData, PasswordHasher},
        shared::{domain_validation::domain_valid, query::DomainJson},
    },
};

domain_valid!(pub struct Request {
    username: UsernameBuf,
    password: String,
    name: Option<String>,
    email: Option<EmailBuf>,
}; UncheckedRequest);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error")]
    Database,
    #[error("Verify error")]
    Tokens,
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[post("/register/")]
pub async fn endpoint(
    encoder: Data<AuthEncoder>,
    req: DomainJson<Request>,
    password_hasher: PasswordHasher,
    register: Register,
) -> Result<impl Responder, Error> {
    let Request {
        username,
        password,
        name,
        email,
    } = req.into_inner();

    let password = password_hasher.hash(password);
    let Ok(register::Response { id, role, name }) = register
        .run(&username, name.as_deref(), email, &password)
        .await
    else {
        return Err(Error::Database);
    };

    let agent_response = account::AccountResponse {
        id,
        kind: role,
        name,
        username,
    };

    let tokens = encoder
        .generate_tokens(ClaimsData::user_claims(id))
        .map_err(|_| Error::Tokens)?;

    let response = super::login::Response {
        user: agent_response,
        refresh_token: tokens.refresh_token,
        token: tokens.token,
    };

    Ok(HttpResponse::Created().json(response))
}
