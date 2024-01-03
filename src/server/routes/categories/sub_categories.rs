use actix_web::web::{scope, ServiceConfig};

mod get_all {
    use actix_web::{
        get,
        web::{Data, Path},
        Responder,
    };

    use crate::{
        domain::category, persistence::db::Pool, server::shared::response::select_response,
    };

    #[get("/")]
    pub async fn endpoint(pool: Data<Pool>, path: Path<uuid::Uuid>) -> impl Responder {
        let id = path.into_inner();
        let result = category::get_sub_categories_by_category(pool.get_ref(), id).await;
        select_response(result)
    }
}

mod create_one {
    use actix_web::{
        post,
        web::{Data, Path},
        Responder,
    };

    use crate::{
        domain::{admin::IsAdminFactory, category},
        persistence::db::Pool,
        server::shared::{query::ValidJson, response::insert_response},
    };

    #[derive(serde::Deserialize, validator::Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Request {
        #[validate(length(min = 1))]
        name: String,
    }

    #[post("/", wrap = "IsAdminFactory")]
    pub async fn endpoint(
        pool: Data<Pool>,
        req: ValidJson<Request>,
        path: Path<uuid::Uuid>,
    ) -> impl Responder {
        let id = path.into_inner();
        let Request { name } = req.into_inner();

        let result = category::create_subcategory(pool.get_ref(), &name, id).await;

        insert_response(result)
    }
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("{id}/sub-categories")
            .service(get_all::endpoint)
            .service(create_one::endpoint),
    );
}
