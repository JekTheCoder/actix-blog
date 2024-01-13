pub use db::{by_id};

pub mod value_objects;
pub mod features;

mod models {
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Serialize, Debug)]
    pub struct User {
        pub id: Uuid,
        pub account_id: Uuid,
        pub email: Option<String>,
    }

    pub struct CreateRequest {
        pub username: String,
        pub password: String,
        pub name: String,
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
        domain::user::models::PublicUser,
        persistence::db::{
            entities::{IdSelect, SelectErr},
            Pool,
        },
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
