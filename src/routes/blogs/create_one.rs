use actix_web::{post, web::Data, Responder};

use crate::{
    modules::{admin::AdminId, blog::{self, BlogParse}, db::Pool},
    shared::extractors::valid_json::ValidJson,
    sqlx::void_insert_response,
};

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Request {
    #[validate(length(min = 1))]
    pub content: String,
}

#[post("/")]
pub async fn endpoint(
    pool: Data<Pool>,
    req: ValidJson<Request>,
    AdminId { id }: AdminId,
) -> impl Responder {
    let Request { content } = req.as_ref();

    let BlogParse {
        title,
        content: html_content,
    } = blog::parse(content).expect("Foo");

    let result = blog::create(pool.get_ref(), id, &title, content, &html_content).await;
    void_insert_response(result)
}
