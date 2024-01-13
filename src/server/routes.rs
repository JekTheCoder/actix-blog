mod auth;
mod blogs;
mod categories;
mod comments;
mod sub_categories;
mod tags;

use actix_web::web::ServiceConfig;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.configure(auth::router)
        .configure(blogs::router)
        .configure(comments::router)
        .configure(categories::router)
        .configure(sub_categories::router)
        .configure(tags::router);
}
