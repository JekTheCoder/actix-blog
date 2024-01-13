use actix_web::web::Data;
use sqlx::query_as;
use uuid::Uuid;

use crate::{
    domain::user::value_objects::{Role, UsernameBuf},
    persistence::db::Pool,
    server::service::sync_service,
};

sync_service!(Login; pool: Data<Pool>);

pub enum Error {
    NotFound,
    Password,
    Database,
}

pub struct AccountData {
    pub id: Uuid,
    pub username: String,
    pub password: String,

    pub name: String,
    pub kind: Role,
}

pub struct Response {
    pub id: Uuid,
    pub username: UsernameBuf,

    pub name: String,
    pub kind: Role,
}

impl From<AccountData> for Response {
    fn from(value: AccountData) -> Self {
        Self {
            id: value.id,
            username: UsernameBuf::from_boxed_unchecked(value.username.into_boxed_str()), 
            // We know it's a valid username
            name: value.name,
            kind: value.kind,
        }
    }
}

impl Login {
    pub async fn run(&self, username: &UsernameBuf, password: &str) -> Result<Response, Error> {
        let account = match query_as!(
            AccountData,
            r#"SELECT id, username, password, name, kind AS "kind: _" FROM accounts WHERE username = $1;"#,
            username.as_str()
        )
        .fetch_optional(self.pool.as_ref())
        .await {
            Ok(Some(account)) => account,
            Ok(None) => return Err(Error::NotFound),
            Err(_) => return Err(Error::Database),
        };

        bcrypt::verify(password, &account.password).map_err(|_| Error::Password)?;

        Ok(account.into())
    }
}
