use actix_web::ResponseError;

use crate::server::shared::response::JsonResponse;

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
