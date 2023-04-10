use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::models::tokens::Tokens;

use super::{claims::Claims, error::JwtEncodeError};

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

    pub fn generate_tokens(&self, id: uuid::Uuid) -> Result<Tokens, JwtEncodeError> {
        let auth_token = self.auth(id)?;
        let refresh_token = self.refresh(id)?;

        Ok(Tokens {
            token: auth_token,
            refresh_token,
        })
    }
}
