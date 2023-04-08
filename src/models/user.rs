use validator::Validate;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub name: String,
    pub email: Option<String>,
    pub id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct CreateReq {
    #[validate(length(min=1))]
    pub username: String,
    #[validate(length(min=1))]
    pub password: String,
    #[validate(length(min=1))]
    pub name: String,
    #[validate(email(message="email not valid"))]
    pub email: Option<String>,
}
