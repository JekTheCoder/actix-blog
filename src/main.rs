mod shared;

mod error;
mod routes;
mod services;
mod traits;
mod utils;

mod actix;
mod modules;

use actix_cors::Cors;
use actix_web::{
    middleware::{NormalizePath, TrailingSlash},
    App, HttpServer,
};
use thiserror::Error;

use crate::{actix::AppConfigurable, modules::db::DbConfig};

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

    let host = dotenvy::var("HOST").expect("HOST could not load");
    let cors_host = dotenvy::var("CORS_HOST").expect("CORS_HOST could not load");

    let db_config = DbConfig::new().await;
    println!("Host: {}", &host);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cors_host)
            .allow_any_method()
            .allow_any_header();

        let app = App::new()
            .wrap(cors)
            .use_config(db_config.clone())
            .configure(modules::auth::configure)
            .configure(routes::router)
            .wrap(NormalizePath::new(TrailingSlash::Always));

        println!("󱓞󱓞 ¡Blazingly fazt! 󱓞󱓞");
        app
    })
    .bind(host)?
    .run()
    .await
    .map_err(|err| InitError::Io(err))
}
