mod create_one;
mod get_all;
mod get_image;
mod get_one;
mod update_one;
mod upload_images;

mod comments;
mod tags;

use actix_web::web::{scope, ServiceConfig};

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/blogs")
            .service(create_one::endpoint)
            .service(get_all::endpoint)
            .service(get_one::endpoint)
            .service(upload_images::endpoint)
            .service(get_image::endpoint)
            .service(update_one::endpoint)
            .configure(comments::router)
            .configure(super::comments::router)
            .configure(tags::router),
    );
}
