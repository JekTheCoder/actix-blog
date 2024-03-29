use actix_web::web::{scope, ServiceConfig};

mod delete {
    use actix_web::{
        delete,
        web::{Data, Path},
        Responder,
    };
    use uuid::Uuid;

    use crate::{
        domain::blog_grouping,
        persistence::db::Pool,
        server::{admin::IsAdminFactory, shared::response::deleted_response},
    };

    #[delete("/{id}/", wrap = "IsAdminFactory")]
    pub async fn endpoint(pool: Data<Pool>, id: Path<Uuid>) -> impl Responder {
        let result = blog_grouping::delete_tag(pool.get_ref(), id.into_inner()).await;
        deleted_response(result)
    }
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/tags").service(delete::endpoint));
}
