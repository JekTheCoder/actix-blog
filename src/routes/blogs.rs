use actix_web::{
    post,
    web::{scope, Json, ServiceConfig},
    Responder,
};
use sqlx::query_as;
use validator::Validate;

use crate::{error::http::json::JsonResponse, extractors::auth::AuthUser, models::blog::CreateReq};

#[post("/")]
async fn create_one(req: Json<CreateReq>, user: AuthUser) -> actix_web::Result<impl Responder> {
    req.validate().map_err(|json| JsonResponse::body(json))?;
    

    Ok("")
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("blogs").service(create_one));
}
