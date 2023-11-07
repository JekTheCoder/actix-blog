use actix_web::{post, web::Data, Responder};

use crate::{
    modules::{category, db::Pool},
    shared::extractors::valid_json::ValidJson,
    sqlx::insert_response,
};

#[derive(serde::Deserialize, validator::Validate)]
pub struct Request {
    #[validate(length(min = 1))]
    name: String,
}

#[post("/")]
pub async fn endpoint(pool: Data<Pool>, request: ValidJson<Request>) -> impl Responder {
    let Request { name } = request.into_inner();
    let response = category::create_category(pool.as_ref(), &name).await;
    insert_response(response)
}
