use actix_web::HttpResponse;
use serde::Serialize;

use super::into_http_err::IntoHttpErr;

pub trait JsonResult {
    fn json_result(self) -> Result<HttpResponse, actix_web::Error>;
}

impl<T: Serialize, E: IntoHttpErr> JsonResult for Result<T, E> {
    fn json_result(self) -> Result<HttpResponse, actix_web::Error> {
        match self {
            Ok(t) => Ok(HttpResponse::Ok().json(t)),
            Err(e) => Err(e.http_err().into()),
        }
    }
}

//impl<T: Serialize, E: IntoHttpErr> JsonResult for Result<Vec<T>, E> {
//    fn json_result(self) -> Result<HttpResponse, actix_web::Error> {
//        match self {
//            Ok(t) => Ok(HttpResponse::Ok().json(t)),
//            Err(e) => Err(e.http_err().into()),
//        }
//    }
//}
