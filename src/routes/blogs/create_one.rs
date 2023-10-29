use actix_web::{post, web::Data, Responder};

use crate::{
    modules::{admin::AdminId, db::Pool},
    shared::{db::models::blogs, extractors::valid_json::ValidJson},
    traits::into_response::IntoResponse,
};

use super::request::BlogCreateReq;

#[post("/")]
pub async fn create_one(
    pool: Data<Pool>,
    req: ValidJson<BlogCreateReq>,
    AdminId { id }: AdminId,
) -> actix_web::Result<impl Responder> {
    let BlogCreateReq { title, content } = req.as_ref();

    let parser = pulldown_cmark::Parser::new(&content);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    blogs::create(pool.get_ref(), id, title, content, &html_output)
        .await
        .into_response()
}
