use actix_web::{dev::Payload, web::Query, FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use std::{
    fmt::Debug,
    future::{ready, Ready}, ops::{Deref, DerefMut},
};

use crate::traits::partial_default::PartialDefault;

#[derive(Debug)]
pub struct PartialQuery<T: Debug + PartialDefault>(T);

impl<T: Debug> FromRequest for PartialQuery<T>
where
    T: PartialDefault,
    T::Partial: DeserializeOwned,
{
    type Future = Ready<Result<PartialQuery<T>, Self::Error>>;
    type Error = <Query<T::Partial> as FromRequest>::Error;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let res = Query::<T::Partial>::from_request(req, payload)
            .into_inner()
            .map(|query| {
                let inner = query.into_inner();

                Self(<T as PartialDefault>::from_partial(inner))
            });

        ready(res)
    }
}

impl<T: Debug + PartialDefault> Deref for PartialQuery<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug + PartialDefault> DerefMut for PartialQuery<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug + PartialDefault> PartialQuery<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

