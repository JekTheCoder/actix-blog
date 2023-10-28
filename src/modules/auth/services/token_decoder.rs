use jsonwebtoken::{decode, DecodingKey, Validation};

use super::super::models::claims::Claims;

#[derive(Clone)]
pub struct TokenDecoder {
    key: DecodingKey,
    validation: Validation,
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
