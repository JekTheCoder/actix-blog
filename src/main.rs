mod shared;

mod error;
mod routes;
mod traits;
mod utils;

mod actix;
mod modules;
mod sqlx;

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
    let cors_hosts_ = dotenvy::var("CORS_HOSTS").expect("CORS_HOSTS could not load");

    let cors_hosts =
        serde_json::from_str::<Vec<String>>(&cors_hosts_).expect("could not parse CORS_HOSTS as JSON");

    let db_config = DbConfig::new().await;

    println!("Host: {}", &host);

    HttpServer::new(move || {
        let cors = {
            let mut cors = Cors::default()
                .allow_any_method()
                .allow_any_header();

            for host in cors_hosts.iter() {
                cors = cors.allowed_origin(host);
            }

            cors
        };

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
