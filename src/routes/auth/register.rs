use actix_web::{http::StatusCode, post, web::Data, HttpResponse, Responder, ResponseError};

use crate::{
    modules::{
        account,
        auth::{AuthEncoder, ClaimsData, Role},
        db::Pool,
        user,
    },
    shared::{extractors::valid_json::ValidJson, models::insert_return::IdSelect},
};

use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Request {
    #[validate(length(min = 1), custom(function = "validate_username"))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(email(message = "email not valid"))]
    pub email: Option<String>,
}

fn validate_username(username: &str) -> Result<(), validator::ValidationError> {
    if username.contains(|c: char| {
        !c.is_ascii()
            || c.is_whitespace()
            || c.is_ascii_punctuation()
            || c.is_ascii_control()
            || c == '@'
    }) {
        return Err(validator::ValidationError::new("invalid"));
    }

    Ok(())
}

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
    pool: Data<Pool>,
    encoder: Data<AuthEncoder>,
    req: ValidJson<Request>,
) -> Result<impl Responder, Error> {
    let Request {
        username,
        password,
        name,
        email,
    } = req.into_inner();

    let name = match name {
        Some(name) => name,
        None => username.clone(),
    };

    let mut req = user::CreateRequest {
        username,
        password,
        name,
        email,
    };

    let password = bcrypt::hash(req.password, bcrypt::DEFAULT_COST).unwrap();

    req.password = password;

    let IdSelect { id } = user::create(pool.get_ref(), &req)
        .await
        .map_err(|_| Error::Database)?;

    let user::CreateRequest { name, username, .. } = req;
    let agent_response = account::AccountResponse {
        id,
        kind: Role::User,
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
