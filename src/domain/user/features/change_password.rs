use actix_web::web::Data;
use uuid::Uuid;

use crate::{
    persistence::db::Pool,
    server::{auth::HashedPassword, service::sync_service},
};

sync_service!(ChangePassword; pool: Data<Pool>);

pub enum Error {
    NotFound,
    NotEqual,
    Internal,
}

struct AccountData {
    password: String,
}

impl ChangePassword {
    pub async fn run(
        &self,
        account_id: Uuid,
        new_password: &HashedPassword,
        old_pasword: &str,
    ) -> Result<(), Error> {
        let account = match sqlx::query_as!(
            AccountData,
            "SELECT password FROM accounts WHERE id = $1",
            account_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
        {
            Ok(Some(account)) => account,
            Ok(None) => return Err(Error::NotFound),
            Err(_) => return Err(Error::Internal),
        };

        let equal = bcrypt::verify(old_pasword, &account.password).unwrap();
        if !equal {
            return Err(Error::NotEqual);
        }

        if sqlx::query!(
            "UPDATE accounts SET password = $1 WHERE id = $2",
            new_password.as_ref(),
            account_id
        )
        .execute(self.pool.as_ref())
        .await
        .is_err()
        {
            return Err(Error::Internal);
        }

        Ok(())
    }
}
