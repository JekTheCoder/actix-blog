use crate::modules::user;
use crate::shared::db::Pool;
use crate::traits::json_result::JsonResult;
use actix_web::web::Data;
use actix_web::{
    get,
    web::{scope, Path, ServiceConfig},
    Responder,
};

#[get("/{id}/")]
async fn get_one(pool: Data<Pool>, id: Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    user::by_id(pool.get_ref(), id).await.json_result()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/users").service(get_one));
}
