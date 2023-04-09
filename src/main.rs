mod app;
mod routes;
mod models;
mod error;
mod traits;
mod services;

use actix_web::{HttpServer, App, web, middleware::{TrailingSlash, NormalizePath}};
use sqlx::postgres::PgPoolOptions;
use app::AppState;
use thiserror::Error;

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
    let encoder = crate::services::auth::AuthEncoder::default();
    let decoder = crate::services::auth::AuthDecoder::default();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(encoder.clone()))
            .app_data(web::Data::new(decoder.clone()))
            .configure(routes::router)
            .wrap(NormalizePath::new(TrailingSlash::Always))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|err| InitError::Io(err))
}
