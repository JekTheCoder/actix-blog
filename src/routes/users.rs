use crate::{app::AppData, traits::into_http::IntoHttp};
use actix_web::{get, web::{ServiceConfig, Path, scope}, Responder};
use sqlx::query_as;
use crate::models::user;

#[get("/{id}/")]
async fn get_one(app: AppData, id: Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    query_as!(
        user::Response,
        "SELECT username, name, id FROM users WHERE id = $1",
        id
    )
    .fetch_one(&app.pool)
    .await
    .ok()
    .into_http()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/users").service(get_one));
}
