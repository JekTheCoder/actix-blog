use serde::Serialize;
use uuid::Uuid;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct AdminId(Uuid);

impl AdminId {
    pub const fn unchecked_new(id: Uuid) -> Self {
        Self(id)
    }

    pub const fn into_inner(self) -> Uuid {
        self.0
    }
}

impl From<AdminId> for Uuid {
    fn from(id: AdminId) -> Self {
        id.0
    }
}

pub mod from_request {
    use std::fmt::Display;

    use actix_web::{FromRequest, ResponseError};
    use uuid::Uuid;

    use crate::{
        domain::user::features::convert_to_admin_id::ConvertToAdminId,
        server::{
            admin::uncheked_admin_id::{get_unchecked_admin_id, Error as UncheckedAdminError},
            service::FromRequestSync,
        },
        shared::future::DynFuture,
    };

    use super::AdminId;

    #[derive(Debug)]
    pub enum Error {
        Claims,
        NotAdmin,
        Service,
    }

    impl From<UncheckedAdminError> for Error {
        fn from(err: UncheckedAdminError) -> Self {
            match err {
                UncheckedAdminError::Claims => Self::Claims,
                UncheckedAdminError::NotAdmin => Self::NotAdmin,
            }
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::Claims => write!(f, "Claims error"),
                Error::NotAdmin => write!(f, "Not admin"),
                Error::Service => write!(f, ""),
            }
        }
    }

    impl ResponseError for Error {
        fn status_code(&self) -> actix_web::http::StatusCode {
            match self {
                Error::Claims => actix_web::http::StatusCode::UNAUTHORIZED,
                Error::NotAdmin => actix_web::http::StatusCode::FORBIDDEN,
                Error::Service => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }

    impl AdminId {
        pub async fn from_req(req: &actix_web::HttpRequest) -> Result<Self, Error> {
            let (command, id) = build_command(req)?;
            command.run(id).await.map_err(|_| Error::NotAdmin)
        }
    }

    impl FromRequest for AdminId {
        type Error = Error;
        type Future = DynFuture<Result<Self, Self::Error>>;

        fn from_request(
            req: &actix_web::HttpRequest,
            _: &mut actix_web::dev::Payload,
        ) -> Self::Future {
            let Ok((command, id)) = build_command(req) else {
                return Box::pin(async { Err(Error::Claims) });
            };

            Box::pin(async move { command.run(id).await.map_err(|_| Error::NotAdmin) })
        }
    }

    fn build_command(req: &actix_web::HttpRequest) -> Result<(ConvertToAdminId, Uuid), Error> {
        let id = get_unchecked_admin_id(req)?;

        match ConvertToAdminId::sync_from_request(req) {
            Ok(command) => Ok((command, id)),
            Err(_) => Err(Error::Service),
        }
    }
}
