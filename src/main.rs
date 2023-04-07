use actix_web::{
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, ResponseError,
};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, query_as, types::Uuid, PgPool};
use std::fmt::Display;
use thiserror::Error;

use dotenvy;

#[derive(Serialize)]
struct User {
    username: String,
    password: String,
    name: String,
    email: Option<String>,
    id: Uuid,
}

#[derive(Debug)]
struct AppError;
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NOOOOOOOOOOOOOOOOO")
    }
}
impl ResponseError for AppError {}

#[get("/hello")]
async fn hello() -> impl Responder {
    "world"
}

#[get("/users")]
async fn get_all(app: Data<AppState>) -> actix_web::Result<impl Responder> {
    let users: Vec<_> = query_as!(User, "SELECT * FROM users")
        .fetch_all(&app.pool)
        .await
        .map_err(|_| AppError)?;

    Ok(web::Json(users))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Debug, Error)]
enum InitError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Env(#[from] dotenvy::Error)
}

#[actix_web::main]
async fn main() -> Result<(), InitError> {
    dotenvy::dotenv()?;

    let database = dotenvy::var("DATABASE_URL").expect("DATABASE could not load"); 

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database)
        .await
        .expect("Pg pool not conected");
    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(hello)
            .service(get_all)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|err| InitError::Io(err))
}
