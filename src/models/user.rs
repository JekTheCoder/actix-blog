use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub name: String,
    pub email: Option<String>,
    pub id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct CreateReq {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email(message = "email not valid"))]
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub username: String,
    pub name: String,
    pub id: Uuid,
}

impl From<User> for Response {
    fn from(value: User) -> Self {
        Self {
            username: value.username,
            name: value.name,
            id: value.id,
        }
    }
}
