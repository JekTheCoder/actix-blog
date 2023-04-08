use crate::{
    app::AppData,
    error::http::internal::InternalError,
    models::user::{CreateReq, User, self},
    traits::into_http::IntoHttp,
};
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use sqlx::{query, query_as};
use validator::Validate;

#[get("/")]
async fn get_all(app: AppData) -> actix_web::Result<impl Responder> {
    let users: Vec<_> = query_as!(User, "SELECT * FROM users")
        .fetch_all(&app.pool)
        .await
        .map_err(|_| InternalError)?;

    Ok(web::Json(users))
}

#[post("/")]
async fn post_one(app: AppData, req: web::Json<CreateReq>) -> impl Responder {
    if let Err(validate) = req.validate() {
        return HttpResponse::BadRequest().json(validate);
    };

    let CreateReq {
        username,
        password,
        name,
        email,
    } = req.0;
    let password = match bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
        Ok(p) => p,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    query!(
        "INSERT INTO users(username, password, name, email) VALUES($1, $2, $3, $4)",
        username,
        password,
        name,
        email
    )
    .execute(&app.pool)
    .await
    .into_http()
}

#[get("/{id}/")]
async fn get_one(app: AppData, id: web::Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    query_as!(user::Response, "SELECT username, name, id FROM users WHERE id = $1", id)
    .fetch_one(&app.pool)
    .await
    .ok()
    .into_http()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
        .service(get_all)
        .service(post_one)
        .service(get_one)
    );
}
