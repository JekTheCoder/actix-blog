mod get_all;
mod create;

use actix_web::web::{scope, ServiceConfig};


pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{blog_id}/comments")
            .service(get_all::endpoint)
            .service(create::endpoint),
    );
}
