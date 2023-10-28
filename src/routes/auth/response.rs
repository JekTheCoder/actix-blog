use serde::Serialize;

use crate::{shared::db::models::agents, modules::auth::Tokens};

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
