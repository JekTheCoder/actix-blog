use std::ops::Deref;
use super::token_decoder::TokenDecoder;

#[derive(Clone)]
pub struct RefreshDecoder(TokenDecoder);
impl Deref for RefreshDecoder {
    type Target = TokenDecoder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for RefreshDecoder {
    fn default() -> Self {
        let secret = dotenvy::var("JWT_REFRESH_SECRET").expect("could not load refresh secret");
        Self(TokenDecoder::new(secret))
    }
}
