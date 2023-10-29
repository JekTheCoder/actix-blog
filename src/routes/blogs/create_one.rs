use actix_web::{post, web::Data, Responder};

use crate::{
    modules::{admin::AdminId, blog, db::Pool},
    shared::extractors::valid_json::ValidJson,
    sqlx::void_insert_response,
};

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct Request {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}

#[post("/")]
pub async fn endpoint(
    pool: Data<Pool>,
    req: ValidJson<Request>,
    AdminId { id }: AdminId,
) -> impl Responder {
    let Request { title, content } = req.as_ref();

    let parser = pulldown_cmark::Parser::new(&content);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let result = blog::create(pool.get_ref(), id, title, content, &html_output).await;
    void_insert_response(result)
}
