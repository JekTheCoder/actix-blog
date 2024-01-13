mod login;
mod refresh;
mod register;

use actix_web::web::{scope, ServiceConfig};

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(login::endpoint)
            .service(register::endpoint)
            .service(refresh::endpoint),
    );
}
