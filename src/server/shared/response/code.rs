use actix_web::{http::StatusCode, HttpResponse, Responder, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("response {0}")]
pub struct HttpCode(StatusCode);

impl ResponseError for HttpCode {
    fn status_code(&self) -> StatusCode {
        self.0
    }
}

impl Responder for HttpCode {
    type Body = ();
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::with_body(self.0, ())
    }
}

impl HttpCode {
    pub fn internal_error() -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn conflict() -> Self {
        Self(StatusCode::CONFLICT)
    }

    pub fn not_found() -> Self {
        Self(StatusCode::NOT_FOUND)
    }
}
