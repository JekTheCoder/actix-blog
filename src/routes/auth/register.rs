use actix_web::{http::StatusCode, post, web::Data, HttpResponse, Responder, ResponseError};

use crate::{
    modules::{
        auth::{AuthEncoder, ClaimsData, Role},
        db::Pool,
        user,
    },
    shared::{
        db::models::agents, extractors::valid_json::ValidJson, models::insert_return::IdSelect,
    },
};

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
    req: ValidJson<user::CreateRequest>,
) -> Result<impl Responder, Error> {
    let mut req = req.into_inner();
    let password = bcrypt::hash(req.password, bcrypt::DEFAULT_COST).unwrap();

    req.password = password;

    let IdSelect { id } = user::create(pool.get_ref(), &req)
        .await
        .map_err(|_| Error::Database)?;

    let user::CreateRequest { name, username, .. } = req;
    let agent_response = agents::AgentResponse {
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
