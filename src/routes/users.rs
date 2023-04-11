use crate::models::user::User;
use crate::{app::AppData, traits::into_http::IntoHttp};
use actix_web::{
    get,
    web::{scope, Path, ServiceConfig},
    Responder,
};

#[get("/{id}/")]
async fn get_one(app: AppData, id: Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    User::by_id(&app.pool, id).await.ok().into_http()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/users").service(get_one));
}
