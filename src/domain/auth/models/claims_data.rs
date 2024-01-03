use super::role::Role;

#[derive(Debug, Clone)]
pub struct ClaimsData {
    pub id: uuid::Uuid,
    pub role: Role,
}

impl ClaimsData {
    pub const fn user_claims(id: uuid::Uuid) -> Self {
        Self {
            id,
            role: Role::User,
        }
    }
}
