use serde::Serialize;

use super::{tokens::Tokens, user};

#[derive(Serialize)]
pub struct LoginResponse {
    pub user: user::Response,
    pub token: String,
    pub refresh_token: String,
}

impl LoginResponse {
    pub fn new(user: user::Response, tokens: Tokens) -> Self {
        Self {
            user,
            token: tokens.token,
            refresh_token: tokens.refresh_token,
        }
    }
}
