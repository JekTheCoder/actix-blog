use jsonwebtoken::{decode, DecodingKey, Validation};
use std::ops::Deref;

use super::claims::Claims;

#[derive(Clone)]
pub struct TokenDecoder {
    key: DecodingKey,
    validation: Validation,
}

impl Default for TokenDecoder {
    fn default() -> Self {
        let secret = dotenvy::var("JWT_SECRET").expect("JWT SECRET not found");
        let key = DecodingKey::from_secret(secret.as_bytes());
        Self {
            key,
            validation: Validation::default(),
        }
    }
}

impl TokenDecoder {
    pub fn new(secret: impl AsRef<[u8]>) -> Self {
        let key = DecodingKey::from_secret(secret.as_ref());
        Self {
            key,
            validation: Validation::default(),
        }
    }

    pub fn decode(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode::<Claims>(token, &self.key, &self.validation).map(|claims| claims.claims)
    }
}

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
        Self(TokenDecoder::new(&secret))
    }
}

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
