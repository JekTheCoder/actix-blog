use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
    pub token: String,
    pub refresh_token: String,
}
