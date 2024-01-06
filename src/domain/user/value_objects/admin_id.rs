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
    fn from(value: AdminId) -> Self {
        value.0
    }
}

pub mod from_request {
    use std::{fmt::Display, future::ready};

    use actix_web::{FromRequest, ResponseError};
    use uuid::Uuid;

    use crate::{
        domain::user::features::convert_to_admin_id::ConvertToAdminId,
        server::auth::{Claims, Role},
        shared::future::DynFuture,
    };

    use super::AdminId;

    #[derive(Debug)]
    enum Error {
        Claims,
        NotAdmin,
        Service,
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

    impl FromRequest for AdminId {
        type Error = Error;
        type Future = DynFuture<Result<Self, Self::Error>>;

        fn from_request(
            req: &actix_web::HttpRequest,
            payload: &mut actix_web::dev::Payload,
        ) -> Self::Future {
            let id = match get_id(req) {
                Ok(id) => id,
                Err(err) => return Box::pin(ready(Err(err))),
            };

            Box::pin(run_check(req, payload, id))
        }
    }

    async fn run_check(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
        id: Uuid,
    ) -> Result<AdminId, Error> {
        let Ok(command) = ConvertToAdminId::from_request(req, payload).await else {
            return Err(Error::Service);
        };

        command.run(id).await.map_err(|_| Error::NotAdmin)
    }

    fn get_id(req: &actix_web::HttpRequest) -> Result<Uuid, Error> {
        let Ok(Claims { exp, id, role }) = Claims::from_req(req) else {
            return Err(Error::Claims);
        };

        if role != Role::Admin {
            return Err(Error::Claims);
        }

        Ok(id)
    }
}
