mod response;
mod login;
mod route;

use actix_web::web::{scope, ServiceConfig};

use self::{route::{register, refresh}, login::endpoint};



pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(endpoint)
            .service(register)
            .service(refresh),
    );
}
