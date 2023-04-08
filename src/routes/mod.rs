pub mod users;
use actix_web::web::ServiceConfig;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.configure(users::router);
}
