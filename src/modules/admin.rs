pub use admin_id::AdminId;

mod admin_id {
    use std::future;

    use actix_web::{http::StatusCode, FromRequest, ResponseError, web::Data};
    use uuid::Uuid;

    use crate::{
        modules::{auth::{Claims, Role}, db::Pool},
        utils::future::DynFuture,
    };

    pub struct AdminId {
        pub id: Uuid,
    }

    #[derive(Debug, thiserror::Error)]
    pub enum AdminIdError {
        #[error("Claims invalid")]
        Claimns,
        #[error("Not an admin")]
        NotAdmin,
        #[error("Database error")]
        Database,
        #[error("Not found")]
        NotFound,
    }

    impl ResponseError for AdminIdError {
        fn status_code(&self) -> actix_web::http::StatusCode {
            match self {
                Self::Claimns => StatusCode::UNAUTHORIZED,
                Self::NotAdmin => StatusCode::UNAUTHORIZED,
                Self::Database => StatusCode::INTERNAL_SERVER_ERROR,
                Self::NotFound => StatusCode::UNAUTHORIZED,
            }
        }
    }

    impl FromRequest for AdminId {
        type Error = AdminIdError;
        type Future = DynFuture<Result<Self, Self::Error>>;

        fn from_request(
            req: &actix_web::HttpRequest,
            _: &mut actix_web::dev::Payload,
        ) -> Self::Future {
            let claims = match Claims::from_req(req) {
                Ok(claims) => claims,
                Err(_) => return Box::pin(future::ready(Err(AdminIdError::Claimns))),
            };

            if claims.role != Role::Admin {
                return Box::pin(future::ready(Err(AdminIdError::NotAdmin)));
            }

            let pool = req
                .app_data::<Data<Pool>>()
                .expect("Pool not found")
                .clone();

            Box::pin(async move {
                match by_account_id(&pool, claims.id).await {
                    Ok(Some(admin_id)) => Ok(admin_id),
                    Ok(None) => Err(AdminIdError::NotFound),
                    Err(_) => Err(AdminIdError::Database),
                }
            })
        }
    }

    async fn by_account_id<'a>(
        pool: &'a Pool,
        account_id: Uuid,
    ) -> Result<Option<AdminId>, sqlx::Error> {
        sqlx::query_as!(
            AdminId,
            "SELECT id FROM admins WHERE account_id = $1",
            account_id
        )
        .fetch_optional(pool)
        .await
    }
}
