use core::fmt::Debug;
use std::fmt::Display;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub struct JsonResponse<T: Debug>(T);

impl<T: Serialize + Debug> Display for JsonResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.serialize(f)
    }
}

impl<T: Serialize + Debug + Clone> ResponseError for JsonResponse<T> {
    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::BadRequest().json(self.0.clone())
    }
}

impl<T: Debug> JsonResponse<T> {
    pub fn body(t: T) -> Self {
        Self(t)
    }
}
