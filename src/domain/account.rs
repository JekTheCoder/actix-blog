pub use db::by_username;
pub use models::{AccountResponse, PublicAccount};

mod models {
    use serde::Serialize;
    use uuid::Uuid;

    use crate::domain::user::value_objects::{Role, UsernameBuf};

    // Common info of an user or an admin
    #[derive(sqlx::FromRow, Clone, Debug)]
    pub struct Account {
        pub id: Uuid,
        pub username: String,
        pub password: String,

        pub name: String,
        pub kind: Role,
    }

    #[derive(Serialize)]
    pub struct AccountResponse {
        pub id: Uuid,
        pub username: UsernameBuf,
        pub name: String,
        pub kind: Role,
    }

    impl From<crate::domain::user::features::login::Response> for AccountResponse {
        fn from(value: crate::domain::user::features::login::Response) -> Self {
            Self {
                id: value.id,
                username: value.username,
                name: value.name,
                kind: value.kind,
            }
        }
    }

    impl From<Account> for AccountResponse {
        fn from(value: Account) -> Self {
            Self {
                id: value.id,
                username: value.username.try_into().unwrap(),
                name: value.name,
                kind: value.kind,
            }
        }
    }

    #[derive(Serialize)]
    pub struct PublicAccount {
        pub id: Uuid,
        pub name: String,
        pub username: String,
    }
}

mod db {
    use sqlx::query_as;

    use crate::persistence::db::Pool;

    use super::models::Account;

    pub async fn by_username(pool: &Pool, username: &str) -> Result<Account, sqlx::Error> {
        query_as!(
            Account,
            r#"SELECT id, username, password, name, kind AS "kind: _" FROM accounts WHERE username = $1;"#,
            username
        )
        .fetch_one(pool)
        .await
    }
}
