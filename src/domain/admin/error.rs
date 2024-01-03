use actix_web::{ResponseError, http::StatusCode};

#[derive(Debug, thiserror::Error)]
pub enum AdminError {
    #[error("Claims invalid")]
    Claimns,
    #[error("Not an admin")]
    NotAdmin,
    #[error("Database error")]
    Database,
    #[error("Not found")]
    NotFound,
}

impl ResponseError for AdminError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::Claimns => StatusCode::UNAUTHORIZED,
            Self::NotAdmin => StatusCode::UNAUTHORIZED,
            Self::Database => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::UNAUTHORIZED,
        }
    }
}
