mod app;
mod error;
mod models;
mod routes;
mod services;
mod traits;

use actix_web::{
    middleware::{NormalizePath, TrailingSlash},
    web::Data,
    App, HttpServer,
};
use app::AppState;
use services::auth::{AuthDecoder, AuthEncoder, RefreshDecoder};
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;

#[derive(Debug, Error)]
enum InitError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Env(#[from] dotenvy::Error),
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
    let encoder = AuthEncoder::default();
    let auth_decoder = AuthDecoder::default();
    let refresh_decoder = RefreshDecoder::default();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .app_data(Data::new(encoder.clone()))
            .app_data(Data::new(auth_decoder.clone()))
            .app_data(Data::new(refresh_decoder.clone()))
            .configure(routes::router)
            .wrap(NormalizePath::new(TrailingSlash::Always))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(|err| InitError::Io(err))
}
