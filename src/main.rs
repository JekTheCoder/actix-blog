mod shared;

mod error;
mod routes;
mod services;
mod traits;
mod utils;

mod modules;

use crate::shared::db::PoolOptions;
use actix_cors::Cors;
use actix_web::{
    middleware::{NormalizePath, TrailingSlash},
    web::Data,
    App, HttpServer,
};
use thiserror::Error;

#[derive(Debug, Error)]
enum InitError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Env(#[from] dotenvy::Error),
}

fn main() -> Result<(), InitError> {
    <::actix_web::rt::System>::new().block_on(run())
}

async fn run() -> Result<(), InitError> {
    dotenvy::dotenv()?;

    let database = dotenvy::var("DATABASE_URL").expect("DATABASE could not load");
    let host = dotenvy::var("HOST").expect("HOST could not load");
    let cors_host = dotenvy::var("CORS_HOST").expect("CORS_HOST could not load");

    let pool = PoolOptions::new()
        .max_connections(10)
        .connect(&database)
        .await
        .expect("Pg pool not conected");

    println!("Host: {}", &host);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cors_host)
            .allow_any_method()
            .allow_any_header();

        let app = App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .configure(modules::auth::configure)
            .configure(routes::router)
            .wrap(NormalizePath::new(TrailingSlash::Always));

        println!("Blazining initialized!!");
        app
    })
    .bind(host)?
    .run()
    .await
    .map_err(|err| InitError::Io(err))
}
