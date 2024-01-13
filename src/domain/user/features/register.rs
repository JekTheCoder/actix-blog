use super::super::value_objects::Role;
use actix_web::web::Data;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{
    domain::user::value_objects::{EmailBuf, UsernameBuf},
    persistence::db::{entities::IdSelect, Pool},
    server::{auth::HashedPassword, service::sync_service},
};

sync_service!(Register; pool: Data<Pool>);

pub struct Response {
    pub id: Uuid,
    pub role: Role,
    pub name: String,
}

impl Register {
    pub async fn run(
        &self,
        username: &UsernameBuf,
        name: Option<&str>,
        email: Option<EmailBuf>,
        password: &HashedPassword,
    ) -> Result<Response, sqlx::Error> {
        let username = username.as_ref().as_ref();
        let name = match name {
            Some(name) => name,
            None => username,
        };

        let mut tx = self.pool.begin().await?;

        let IdSelect { id } = query_as!(
            IdSelect,
            r#"INSERT INTO accounts (username, password, name, kind) VALUES ($1, $2, $3, $4) RETURNING id"#,
            username,
            password.as_ref(),
            name,
            Role::User as Role
        )
        .fetch_one(&mut tx)
        .await?;

        let email = email.as_ref().map(|email| email.as_ref().as_ref());

        query!("INSERT INTO users(id, email) VALUES ($1, $2)", id, email)
            .execute(&mut tx)
            .await?;

        tx.commit().await.unwrap();

        Ok(Response {
            id,
            role: Role::User,
            name: name.to_owned(),
        })
    }
}
