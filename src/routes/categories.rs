mod sub_categories;
mod tags;

mod create_one;

use actix_web::{
    get,
    web::{scope, Data, ServiceConfig},
    Responder,
};

use crate::{
    modules::{category::get_all_categories, db::Pool},
    sqlx::select_response,
};

#[get("/")]
async fn get_all(pool: Data<Pool>) -> impl Responder {
    let result = get_all_categories(pool.as_ref()).await;
    select_response(result)
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/categories")
            .service(get_all)
            .service(create_one::endpoint)
            .configure(sub_categories::router)
            .configure(tags::router),
    );
}
