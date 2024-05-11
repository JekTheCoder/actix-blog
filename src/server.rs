pub mod admin;
mod app_config;
pub mod auth;
mod routes;
pub mod service;
pub mod shared;

pub use app_config::{AppConfig, AppConfigurable};
pub use server::run;

mod server {
    use actix_cors::Cors;
    use actix_web::{
        middleware::{NormalizePath, TrailingSlash},
        App, HttpServer,
    };

    use crate::{
        domain::blog,
        persistence::db::DbConfig,
        persistence::public,
        server::{routes, AppConfigurable},
    };

    pub async fn run() -> Result<(), std::io::Error> {
        if let Err(e) = dotenvy::dotenv() {
            println!("Warning could not load .env file, skipping error: {e}");
        };

        let static_dir = dotenvy::var("STATIC_DIR").expect("could not load STATIC_DIR");
        let static_dir = public::PublicDir::new_data(&static_dir);

        let pkg_dir = {
            let pkg_dir = dotenvy::var("PKG_DIR").expect("could not load PKG_DIR");
            public::PkgDir::new_data(&pkg_dir)
        };

        let host = dotenvy::var("HOST").expect("HOST could not load");

        let db_config = DbConfig::new().await;

        let public_config = public::Config::new(static_dir.clone());
        let blog_config = blog::Config::new(pkg_dir);

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
                .use_config(public_config.clone())
                .use_config(blog_config.clone())
                .configure(super::auth::configure)
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
