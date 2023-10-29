pub use db::{by_id, create};
pub use models::{CreateRequest, PublicUser, User};

mod models {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use validator::Validate;

    #[derive(Serialize, Debug)]
    pub struct User {
        pub id: Uuid,
        pub account_id: Uuid,
        pub email: Option<String>,
    }

    #[derive(Deserialize, Validate)]
    pub struct CreateRequest {
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
    pub struct PublicUser {
        pub id: Uuid,
    }

    impl From<User> for PublicUser {
        fn from(value: User) -> Self {
            Self { id: value.id }
        }
    }
}

mod db {
    use crate::{
        error::sqlx::select::SelectErr,
        modules::{db::Pool, user::models::PublicUser},
        shared::models::insert_return::IdSelect,
    };
    use sqlx::query_as;
    use uuid::Uuid;

    use super::models::CreateRequest;

    pub async fn create(pool: &Pool, req: &CreateRequest) -> Result<IdSelect, sqlx::Error> {
        let CreateRequest {
            username,
            name,
            email,
            password,
        } = req;

        query_as!(
            IdSelect,
            r#"SELECT insert_user($1, $2, $3, $4) AS "id!" "#,
            username,
            password,
            name,
            email.as_ref(),
        )
        .fetch_one(pool)
        .await
    }

    pub async fn by_id(pool: &Pool, id: Uuid) -> Result<PublicUser, SelectErr> {
        query_as!(PublicUser, "SELECT id FROM users WHERE id = $1", id)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }
}
