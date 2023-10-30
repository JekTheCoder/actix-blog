use actix_web::web::{scope, ServiceConfig};

mod get_all {
    use actix_web::{
        get,
        web::{Data, Path},
        Responder,
    };

    use crate::{
        modules::{category, db::Pool},
        sqlx::select_response,
    };

    #[get("/")]
    pub async fn endpoint(pool: Data<Pool>, path: Path<uuid::Uuid>) -> impl Responder {
        let id = path.into_inner();
        let result = category::get_all_sub_categories_by_category(pool.get_ref(), id).await;
        select_response(result)
    }
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("{id}/sub_categories").service(get_all::endpoint));
}
