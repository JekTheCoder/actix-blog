use actix_web::{
    post,
    web::{scope, Data, Json, ServiceConfig},
    Responder,
};
use validator::Validate;

use crate::{db::Pool, models::blog::Blog, traits::catch_http::CatchHttp};
use crate::{extractors::auth::AuthUser, models::blog::CreateReq};

#[post("/")]
async fn create_one(
    pool: Data<Pool>,
    req: Json<CreateReq>,
    user: AuthUser,
) -> actix_web::Result<impl Responder> {
    req.validate().catch_http()?;
    let blog = Blog::create(pool.get_ref(), &req, user.into_inner().id);

    Ok("")
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("blogs").service(create_one));
}
