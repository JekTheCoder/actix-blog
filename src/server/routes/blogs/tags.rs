mod set;

use actix_web::web::{scope, ServiceConfig};

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/{blog_id}/tags").service(set::endpoint));
}
