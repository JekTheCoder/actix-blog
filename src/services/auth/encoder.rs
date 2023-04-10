use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::models::tokens::Tokens;

use super::{claims::Claims, error::JwtEncodeError};

#[derive(Clone)]
pub struct AuthEncoder {
    auth_key: EncodingKey,
    refresh_key: EncodingKey,
    header: Header,
}

impl Default for AuthEncoder {
    fn default() -> Self {
        let secret = dotenvy::var("JWT_SECRET").expect("JWT SECRET not found");
        let refresh_secret =
            dotenvy::var("JWT_REFRESH_SECRET").expect("JWT REFRESH SECRET not found");
        let header = Header::default();

        let key = EncodingKey::from_secret(secret.as_bytes());
        let refresh_key = EncodingKey::from_secret(refresh_secret.as_bytes());

        Self {
            auth_key: key,
            header,
            refresh_key,
        }
    }
}

impl AuthEncoder {
    fn encode(
        &self,
        key: &EncodingKey,
        duration: Duration,
        id: uuid::Uuid,
    ) -> Result<String, JwtEncodeError> {
        let exp = Utc::now()
            .checked_add_signed(duration)
            .expect("Invalid timestamp")
            .timestamp() as usize;
        let claimns = Claims { id, exp };
        encode(&self.header, &claimns, key).map_err(|_| JwtEncodeError)
    }

    pub fn auth(&self, id: uuid::Uuid) -> Result<String, JwtEncodeError> {
        self.encode(&self.auth_key, Duration::minutes(5), id)
    }

    pub fn refresh(&self, id: uuid::Uuid) -> Result<String, JwtEncodeError> {
        self.encode(&self.refresh_key, Duration::weeks(60), id)
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
