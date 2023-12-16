mod create_one;
mod get_all;
mod get_one;
mod upload_images;

mod comments;

use actix_web::web::{scope, ServiceConfig};

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/blogs")
            .service(create_one::endpoint)
            .service(get_all::endpoint)
            .service(get_one::endpoint)
            .configure(comments::router)
            .configure(super::comments::router),
    );
}
