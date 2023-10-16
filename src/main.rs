mod shared;

mod error;
mod routes;
mod services;
mod traits;
mod utils;

use actix_web::{
    middleware::{NormalizePath, TrailingSlash},
    web::Data,
    App, HttpServer,
};
use crate::shared::db::PoolOptions;
use services::auth::{AuthDecoder, AuthEncoder, RefreshDecoder};
use actix_cors::Cors;
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
    let host = dotenvy::var("HOST").expect("HOST could not load");
    let cors_host = dotenvy::var("CORS_HOST").expect("CORS_HOST could not load");

    let pool = PoolOptions::new()
        .max_connections(10)
        .connect(&database)
        .await
        .expect("Pg pool not conected");

    let encoder = AuthEncoder::default();
    let auth_decoder = AuthDecoder::default();
    let refresh_decoder = RefreshDecoder::default();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cors_host)
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(encoder.clone()))
            .app_data(Data::new(auth_decoder.clone()))
            .app_data(Data::new(refresh_decoder.clone()))
            .configure(routes::router)
            .wrap(NormalizePath::new(TrailingSlash::Always))
    })
    .bind(host)?
    .run()
    .await
    .map_err(|err| InitError::Io(err))
}
