use crate::{
    app::AppData,
    error::http::internal::InternalError,
    models::user::{CreateReq, User},
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

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/users").service(get_all).service(post_one));
}
