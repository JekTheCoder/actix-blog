use actix_web::{web::Json, FromRequest};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::utils::future::DynFuture;

pub struct ValidJson<T>(T);

impl<T> AsRef<T> for ValidJson<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> ValidJson<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> FromRequest for ValidJson<T>
where
    T: Validate + DeserializeOwned + 'static,
{
    type Error = actix_web::Error;
    type Future = DynFuture<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let json = Json::<T>::from_request(req, payload);
        Box::pin(validate_json(json))
    }
}

async fn validate_json<T>(json: <Json<T> as FromRequest>::Future) -> actix_web::Result<ValidJson<T>>
where
    T: Validate + DeserializeOwned,
{
    let json = json.await?;
    let value = json.into_inner();
    match value.validate() {
        Ok(_) => Ok(ValidJson(value)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e)),
    }
}
