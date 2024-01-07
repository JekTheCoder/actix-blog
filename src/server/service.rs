use actix_web::ResponseError;
use std::fmt::Display;

pub use from_request_sync::FromRequestSync;

#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal Server Error",)
    }
}

impl ResponseError for Error {}

mod from_request_sync {
    use actix_web::ResponseError;

    pub trait FromRequestSync: Sized {
        type Error: ResponseError;

        fn sync_from_request(req: &actix_web::HttpRequest) -> Result<Self, Self::Error>;
    }

    mod data {
        use std::fmt::Display;

        use actix_web::{web::Data, ResponseError};

        use super::FromRequestSync;

        #[derive(Debug)]
        pub struct Error {}

        impl Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Internal Server Error",)
            }
        }

        impl ResponseError for Error {}

        impl<T: ?Sized + 'static> FromRequestSync for Data<T> {
            type Error = Error;

            fn sync_from_request(req: &actix_web::HttpRequest) -> Result<Self, Self::Error> {
                let data = req.app_data::<Data<T>>();
                match data {
                    Some(data) => Ok(data.clone()),
                    None => Err(Error {}),
                }
            }
        }
    }
}

macro_rules! sync_service {
    ($service_name: ident; $($field_name: ident: $field_type: ty),* ) => {
       pub struct $service_name {
            $($field_name: $field_type),*
        }

        impl crate::server::service::FromRequestSync for $service_name {
            type Error = crate::server::service::Error;

            fn sync_from_request(req: &actix_web::HttpRequest) -> Result<Self, Self::Error> {
                $(
                let Ok($field_name) = <$field_type as crate::server::service::FromRequestSync>::sync_from_request(req) else {
                    return Err(crate::server::service::Error);
                };
                );*

                Ok($service_name {$($field_name),*})
            }
        }

        impl actix_web::FromRequest for $service_name {
            type Error = crate::server::service::Error;
            type Future = std::future::Ready<Result<Self, Self::Error>>;

            fn from_request(
                req: &actix_web::HttpRequest,
                _: &mut actix_web::dev::Payload
            ) -> Self::Future {
                std::future::ready(<Self as crate::server::service::FromRequestSync>::sync_from_request(req))
            }
        }
    };
}

pub(crate) use sync_service;
