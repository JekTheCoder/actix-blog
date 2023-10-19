use serde::Serialize;

use crate::{services::auth::tokens::Tokens, shared::db::models::agents};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub user: agents::AgentResponse,
    pub token: String,
    pub refresh_token: String,
}

impl LoginResponse {
    pub fn new(user: agents::AgentResponse, tokens: Tokens) -> Self {
        Self {
            user,
            token: tokens.token,
            refresh_token: tokens.refresh_token,
        }
    }
}
