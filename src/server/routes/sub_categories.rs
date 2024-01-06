use actix_web::web::{scope, ServiceConfig};

mod get_all {
    use actix_web::{get, web::Data, Responder};

    use crate::{
        domain::category, persistence::db::Pool, server::shared::response::select_response,
    };

    #[get("/")]
    pub async fn endpoint(pool: Data<Pool>) -> impl Responder {
        let result = category::get_all_sub_categories(pool.get_ref()).await;
        select_response(result)
    }
}

mod delete {
    use actix_web::{
        delete,
        web::{Data, Path},
        Responder,
    };
    use uuid::Uuid;

    use crate::{
        domain::category,
        persistence::db::Pool,
        server::{admin::IsAdminFactory, shared::response::deleted_response},
    };

    #[delete("/{id}/", wrap = "IsAdminFactory")]
    pub async fn endpoint(pool: Data<Pool>, id: Path<Uuid>) -> impl Responder {
        let result = category::delete_subcategory(pool.get_ref(), id.into_inner()).await;
        deleted_response(result)
    }
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/sub-categories")
            .service(get_all::endpoint)
            .service(delete::endpoint),
    );
}
