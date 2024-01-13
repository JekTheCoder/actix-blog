use actix_web::{post, web::Data, Responder};

use crate::{
    domain::blog_grouping,
    persistence::db::Pool,
    server::{
        admin::IsAdminFactory,
        shared::{query::ValidJson, response::insert_response},
    },
};

#[derive(serde::Deserialize, validator::Validate)]
pub struct Request {
    #[validate(length(min = 1))]
    name: String,
}

#[post("/", wrap = "IsAdminFactory")]
pub async fn endpoint(pool: Data<Pool>, request: ValidJson<Request>) -> impl Responder {
    let Request { name } = request.into_inner();
    let response = blog_grouping::create_category(pool.as_ref(), &name).await;
    insert_response(response)
}
