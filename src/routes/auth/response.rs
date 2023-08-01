use serde::Serialize;

use crate::{services::auth::tokens::Tokens, shared::db::models::users};

#[derive(Serialize)]
pub struct LoginResponse {
    pub user: users::Response,
    pub token: String,
    pub refresh_token: String,
}

impl LoginResponse {
    pub fn new(user: users::Response, tokens: Tokens) -> Self {
        Self {
            user,
            token: tokens.token,
            refresh_token: tokens.refresh_token,
        }
    }
}
