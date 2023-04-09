use serde::Serialize;

#[derive(Serialize)]
pub struct Tokens {
    pub token: String,
    pub refresh_token: String,
}

