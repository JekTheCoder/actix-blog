pub mod auth;
mod blogs;
mod comments;
pub mod users;

use actix_web::web::ServiceConfig;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.configure(users::router)
        .configure(auth::router)
        .configure(blogs::router)
        .configure(comments::router);
}
