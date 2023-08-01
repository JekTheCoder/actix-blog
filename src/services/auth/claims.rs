use serde::{Deserialize, Serialize};

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
        InnerClaims { id: self.id, role: self.role }
    }
}

#[derive(Debug, Clone)]
pub struct InnerClaims {
    pub id: uuid::Uuid,
    pub role: Role,
}

impl InnerClaims {
    pub const fn new(id: uuid::Uuid, role: Role) -> Self {
        Self { id, role }
    }

    pub const fn user_claims(id: uuid::Uuid) -> Self {
        Self {
            id,
            role: Role::User,
        }
    }

    pub const fn admin_claims(id: uuid::Uuid) -> Self {
        Self {
            id,
            role: Role::Admin,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    Admin,
    User,
}
