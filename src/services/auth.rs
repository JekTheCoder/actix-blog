use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::models::tokens::Tokens;

#[derive(thiserror::Error, Debug)]
#[error("Jwt encode error")]
pub struct JwtEncodeError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    id: uuid::Uuid,
}

#[derive(Clone)]
pub struct AuthEncoder {
    key: EncodingKey,
    header: Header,
}

impl Default for AuthEncoder {
    fn default() -> Self {
        let secret = dotenvy::var("JWT_SECRET").expect("JWT SECRET not found");
        let header = Header::default();

        let key = EncodingKey::from_secret(secret.as_bytes());
        Self { key, header }
    }
}

impl AuthEncoder {
    fn encode(&self, duration: Duration, id: uuid::Uuid) -> Result<String, JwtEncodeError> {
        let exp = Utc::now()
            .checked_add_signed(duration)
            .expect("Invalid timestamp")
            .timestamp() as usize;
        let claimns = Claims { id, exp };
        encode(&self.header, &claimns, &self.key).map_err(|_| JwtEncodeError)
    }

    pub fn auth(&self, id: uuid::Uuid) -> Result<String, JwtEncodeError> {
        self.encode(Duration::minutes(5), id)
    }

    pub fn refresh(&self, id: uuid::Uuid) -> Result<String, JwtEncodeError> {
        self.encode(Duration::weeks(60), id)
    }

    pub fn generate_tokens(
        &self,
        id: uuid::Uuid,
    ) -> Result<Tokens, JwtEncodeError> {
        let auth_token = self.auth(id)?;
        let refresh_token = self.refresh(id)?;

        Ok(Tokens {
            token: auth_token,
            refresh_token,
        })
    }
}

#[derive(Clone)]
pub struct AuthDecoder {
    key: DecodingKey,
    validation: Validation,
}

impl Default for AuthDecoder {
    fn default() -> Self {
        let secret = dotenvy::var("JWT_SECRET").expect("JWT SECRET not found");
        let key = DecodingKey::from_secret(secret.as_bytes());
        Self {
            key,
            validation: Validation::default(),
        }
    }
}

impl AuthDecoder {
    pub fn decode(&self, token: &str) -> Option<uuid::Uuid> {
        decode::<Claims>(token, &self.key, &self.validation)
            .ok()
            .map(|claims| claims.claims.id)
    }
}
