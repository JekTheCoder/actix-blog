use std::future::{ready, Ready};

use actix_web::{error::ErrorUnauthorized, web::Data, FromRequest};
use serde::{Deserialize, Serialize};

use crate::{services::auth::AuthDecoder, utils::http::bearer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub id: uuid::Uuid,
    pub role: Role,
}

impl Claims {
    pub fn new(InnerClaims { id, role }: InnerClaims, exp: usize) -> Self {
        Self { exp, id, role }
    }

    pub const fn inner(self) -> InnerClaims {
        InnerClaims {
            id: self.id,
            role: self.role,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InnerClaims {
    pub id: uuid::Uuid,
    pub role: Role,
}

impl InnerClaims {
    pub const fn user_claims(id: uuid::Uuid) -> Self {
        Self {
            id,
            role: Role::User,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ready(claimns_from_req(req))
    }
}

fn claimns_from_req(
    req: &actix_web::HttpRequest,
) -> Result<Claims, <Claims as FromRequest>::Error> {
    let token = bearer(req).ok_or_else(|| ErrorUnauthorized(""))?;

    let decoder = req
        .app_data::<Data<AuthDecoder>>()
        .expect("Decoder not found");

    decoder.decode(token).map_err(|_| ErrorUnauthorized(""))
}
