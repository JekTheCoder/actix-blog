mod login;
mod register;
mod response;
mod route;

use actix_web::web::{scope, ServiceConfig};

use self::route::refresh;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(login::endpoint)
            .service(register::endpoint)
            .service(refresh),
    );
}
