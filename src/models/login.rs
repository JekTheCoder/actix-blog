use serde::Serialize;

use crate::shared::db::models::users;

use super::tokens::Tokens;

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
