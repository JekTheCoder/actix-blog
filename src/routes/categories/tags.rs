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
        let result = category::get_tags_by_category(pool.get_ref(), id).await;
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
        modules::{category, db::Pool},
        shared::extractors::valid_json::ValidJson,
        sqlx::insert_response,
    };

    #[derive(serde::Deserialize, validator::Validate)]
    #[serde(rename_all = "camelCase")]
    pub struct Request {
        #[validate(length(min = 1))]
        name: String,
        color: String,
    }

    #[post("/")]
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
    cfg.service(
        scope("/tags")
            .service(get_all::endpoint)
            .service(create_one::endpoint),
    );
}
