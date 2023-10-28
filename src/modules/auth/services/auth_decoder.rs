use std::ops::Deref;
use super::token_decoder::TokenDecoder;

#[derive(Clone)]
pub struct AuthDecoder(TokenDecoder);
impl Deref for AuthDecoder {
    type Target = TokenDecoder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for AuthDecoder {
    fn default() -> Self {
        let secret = dotenvy::var("JWT_SECRET").expect("could not load token secret");
        Self(TokenDecoder::new(&secret))
    }
}
