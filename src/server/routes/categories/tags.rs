use actix_web::web::{self, scope, ServiceConfig};

use crate::domain::admin::IsAdminFactory;

mod get_all {
    use actix_web::{
        get,
        web::{Data, Path},
        Responder,
    };

    use crate::{domain::category, persistence::db::Pool, sqlx::select_response};

    #[get("/")]
    pub async fn endpoint(pool: Data<Pool>, path: Path<uuid::Uuid>) -> impl Responder {
        let id = path.into_inner();
        let result = category::get_tags_by_category(pool.get_ref(), id).await;
        select_response(result)
    }
}

mod create_one {
    use actix_web::{
        web::{Data, Path},
        Responder,
    };

    use crate::{
        domain::category, persistence::db::Pool, server::shared::query::ValidJson,
        sqlx::insert_response,
    };

    #[derive(serde::Deserialize, validator::Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Request {
        #[validate(length(min = 1))]
        name: String,
        color: String,
    }

    pub async fn endpoint(
        pool: Data<Pool>,
        req: ValidJson<Request>,
        path: Path<uuid::Uuid>,
    ) -> impl Responder {
        let id = path.into_inner();
        let Request { name, color } = req.into_inner();

        let result = category::create_tag(pool.get_ref(), id, &name, &color).await;

        insert_response(result)
    }
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("{id}/tags").service(get_all::endpoint).route(
        "/",
        web::post().wrap(IsAdminFactory).to(create_one::endpoint),
    ));
}
