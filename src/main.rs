mod shared;

mod error;
mod routes;
mod traits;
mod utils;

mod modules;
mod sqlx;

mod app;
mod persistence;

use actix_cors::Cors;
use actix_web::{
    middleware::{NormalizePath, TrailingSlash},
    App, HttpServer,
};

use crate::{
    app::AppConfigurable,
    modules::{images, server},
    persistence::db::DbConfig,
};

fn main() -> Result<(), std::io::Error> {
    <::actix_web::rt::System>::new().block_on(run())
}

async fn run() -> Result<(), std::io::Error> {
    if let Err(e) = dotenvy::dotenv() {
        println!("Warning could not load .env file, skipping error: {e}");
    };

    let static_dir = dotenvy::var("STATIC_DIR").expect("could not load STATIC_DIR");
    let host = dotenvy::var("HOST").expect("HOST could not load");

    let db_config = DbConfig::new().await;
    let images_config = images::Config::new(static_dir.as_str());
    let server_config = {
        let public_addr = dotenvy::var("PUBLIC_ADDR").expect("could not load PUBLIC_ADDR");
        server::Config::new(&public_addr)
    };

    println!("Host: {}", &host);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .allow_any_origin();

        let app = App::new()
            .wrap(cors)
            .use_config(server_config.clone())
            .use_config(db_config.clone())
            .use_config(images_config.clone())
            .configure(modules::auth::configure)
            .configure(routes::router)
            .wrap(NormalizePath::new(TrailingSlash::Always));

        println!("󱓞󱓞 ¡Blazingly fazt! 󱓞󱓞");

        app
    })
    .bind(host)?
    .run()
    .await
}
