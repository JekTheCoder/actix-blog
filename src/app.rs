mod app_config;
mod routes;

pub use app_config::{AppConfig, AppConfigurable};
pub use server::run;

mod server {
    use actix_cors::Cors;
    use actix_web::{
        middleware::{NormalizePath, TrailingSlash},
        App, HttpServer,
    };

    use crate::{
        app::{routes, AppConfigurable},
        domain::images,
        persistence::db::DbConfig,
    };

    pub async fn run() -> Result<(), std::io::Error> {
        if let Err(e) = dotenvy::dotenv() {
            println!("Warning could not load .env file, skipping error: {e}");
        };

        let static_dir = dotenvy::var("STATIC_DIR").expect("could not load STATIC_DIR");
        let host = dotenvy::var("HOST").expect("HOST could not load");

        let db_config = DbConfig::new().await;
        let images_config = images::Config::new(static_dir.as_str());
        let server_config = {
            let public_addr = dotenvy::var("PUBLIC_ADDR").expect("could not load PUBLIC_ADDR");
            crate::domain::server::Config::new(&public_addr)
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
                .configure(crate::domain::auth::configure)
                .configure(routes::router)
                .wrap(NormalizePath::new(TrailingSlash::Always));

            println!("󱓞󱓞 ¡Blazingly fazt! 󱓞󱓞");

            app
        })
        .bind(host)?
        .run()
        .await
    }
}