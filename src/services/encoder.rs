use actix_web::web::Data;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

#[derive(Clone)]
pub struct Encoder {
    key: EncodingKey,
    header: Header,
}

impl Default for Encoder {
    fn default() -> Self {
        let secret = dotenvy::var("JWT_SECRET").expect("JWT SECRET not found");
        let header = Header::default();

        let key = EncodingKey::from_secret(secret.as_bytes());
        Self { key, header }
    }
}

impl Encoder {
    fn encode<T: Serialize>(&self, to_encode: &T) -> String {
        encode(&self.header, to_encode, &self.key).expect("could not encode")
    }
}

pub type EncoderData = Data<Encoder>;
