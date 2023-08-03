use std::future::{ready, Ready};

use actix_web::{error::ErrorUnauthorized, web::Data, FromRequest};
use serde::{Deserialize, Serialize};

use crate::{
    services::auth::AuthDecoder, shared::db::models::agents::AgentType, utils::http::bearer,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(match self {
            Self::User => 0,
            Self::Admin => 1,
        })
    }
}

impl<'a> Deserialize<'a> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(Self::User),
            1 => Ok(Self::Admin),
            _ => Err(serde::de::Error::custom("Invalid role")),
        }
    }
}

impl From<AgentType> for Role {
    fn from(value: AgentType) -> Self {
        match value {
            AgentType::User => Self::User,
            AgentType::Admin => Self::Admin,
        }
    }
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
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
