use actix_web::{error::ErrorBadRequest, web::Json, FromRequest};

use crate::{server::shared::domain_validation::DomainValid, shared::future::DynFuture};

pub struct DomainJson<T>(T)
where
    T: DomainValid,
    T::Unchecked: serde::de::DeserializeOwned;

impl<T> DomainJson<T>
where
    T: DomainValid,
    T::Unchecked: serde::de::DeserializeOwned,
{
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> FromRequest for DomainJson<T>
where
    T: DomainValid + 'static,
    T::Unchecked: serde::de::DeserializeOwned,
{
    type Error = actix_web::Error;
    type Future = DynFuture<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let json = Json::<T::Unchecked>::from_request(req, payload);
        Box::pin(validate_json(json))
    }
}

async fn validate_json<T>(
    json: <Json<T::Unchecked> as FromRequest>::Future,
) -> Result<DomainJson<T>, <DomainJson<T> as FromRequest>::Error>
where
    T: DomainValid + 'static,
    T::Unchecked: serde::de::DeserializeOwned + 'static,
{
    let unchecked = json.await?.into_inner();
    match <T as DomainValid>::from_unchecked(unchecked) {
        Ok(checked) => Ok(DomainJson(checked)),
        Err(e) => Err(ErrorBadRequest(e)),
    }
}
