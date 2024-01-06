use actix_web::ResponseError;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal Server Error",)
    }
}

impl ResponseError for Error {}

macro_rules! sync_service {
    ($service_name: ident; $($field_name: ident: $field_type: ty),* ) => {
       pub struct $service_name {
            $($field_name: actix_web::web::Data<$field_type>),*
        }

        impl actix_web::FromRequest for $service_name {
            type Error = crate::server::service::Error;
            type Future = std::future::Ready<Result<Self, Self::Error>>;

            fn from_request(
                req: &actix_web::HttpRequest,
                _: &mut actix_web::dev::Payload
            ) -> Self::Future {
                $(
                let Some($field_name) = req.app_data::<actix_web::web::Data<$field_type>>() else {
                    return std::future::ready(Err(crate::server::service::Error));
                };
                );*

                std::future::ready(Ok($service_name {$($field_name: $field_name.clone()),*}))
            }
        }
    };
}

pub(crate) use sync_service;
