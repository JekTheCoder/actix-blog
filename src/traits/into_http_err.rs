use crate::error::http::json::JsonResponse;
use actix_web::ResponseError;

pub trait IntoHttpErr {
    type Err: ResponseError + Into<actix_web::Error>;

    fn http_err(self) -> Self::Err;
}

impl IntoHttpErr for validator::ValidationErrors {
    type Err = JsonResponse<Self>;

    fn http_err(self) -> Self::Err {
        JsonResponse::body(self)
    }
}
