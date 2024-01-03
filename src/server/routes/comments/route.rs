use actix_web::web::{scope, ServiceConfig};

use super::replies;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/comments").configure(replies::router));
}
