use actix_web::{
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    Responder,
};
use validator::Validate;

use super::blog::{Blog, CreateReq};
use crate::{
    db::Pool,
    extractors::{auth::AuthUser, partial_query::PartialQuery},
    models::select_slice::SelectSlice,
    traits::{catch_http::CatchHttp, into_response::IntoResponse, json_result::JsonResult},
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

#[get("/")]
async fn get_all(pool: Data<Pool>, slice: PartialQuery<SelectSlice>) -> impl Responder {
    Blog::get_all(pool.get_ref(), slice.into_inner())
        .await
        .json_result()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/blogs")
            .service(create_one)
            .service(get_all)
            .configure(super::comments::router),
    );
}
