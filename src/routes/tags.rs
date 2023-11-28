use actix_web::web::{scope, ServiceConfig};

mod delete {
    use actix_web::{delete, web::{Data, Path}, Responder};
    use uuid::Uuid;

    use crate::{
        modules::{admin::IsAdminFactory, category, db::Pool},
        sqlx::deleted_response,
    };

    #[delete("/{id}", wrap = "IsAdminFactory")]
    pub async fn endpoint(pool: Data<Pool>, id: Path<Uuid>) -> impl Responder {
        let result = category::delete_tag(pool.get_ref(), id.into_inner()).await;
        deleted_response(result)
    }
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/tags").service(delete::endpoint));
}
