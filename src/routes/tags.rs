use actix_web::web::{scope, ServiceConfig};

mod get_all {
    use actix_web::{get, web::Data, Responder};

    use crate::{
        modules::{category, db::Pool},
        sqlx::select_response,
    };

    #[get("/")]
    pub async fn endpoint(pool: Data<Pool>) -> impl Responder {
        let result = category::get_all_tags(pool.get_ref()).await;
        select_response(result)
    }
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/tags").service(get_all::endpoint));
}
