use actix_web::{
    post,
    web::{scope, Data, Json, ServiceConfig},
    Responder,
};
use validator::Validate;

use crate::{
    db::Pool,
    extractors::auth::AuthUser,
    models::blog::{Blog, CreateReq},
    traits::{catch_http::CatchHttp, into_http::IntoHttp, into_response::IntoResponse},
};

#[post("/")]
async fn create_one(
    pool: Data<Pool>,
    req: Json<CreateReq>,
    user: AuthUser,
) -> actix_web::Result<impl Responder> {
    req.validate().catch_http()?;
    Blog::create(pool.get_ref(), &req, user.into_inner().id)
        .await
        .into_response()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("blogs").service(create_one));
}
