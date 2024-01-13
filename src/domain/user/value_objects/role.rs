use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "account_kind", rename_all = "lowercase")]
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
