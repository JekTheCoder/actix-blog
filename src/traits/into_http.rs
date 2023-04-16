use crate::db::QueryResult;
use actix_web::{HttpResponse, Responder};
use sqlx::postgres::PgQueryResult;

pub trait IntoHttp {
    type Http: Responder;
    fn into_http(self) -> Self::Http;
}

//impl IntoHttp for QueryResult {
//    type Http = HttpResponse;
//
//    fn into_http(self) -> Self::Http {
//        if self.rows_affected() == 0 {
//            HttpResponse::Conflict().finish()
//        } else {
//            HttpResponse::Created().finish()
//        }
//    }
//}

impl IntoHttp for PgQueryResult {
    type Http = HttpResponse;

    fn into_http(self) -> Self::Http {
        if self.rows_affected() == 0 {
            HttpResponse::Conflict().finish()
        } else {
            HttpResponse::Created().finish()
        }
    }
}
