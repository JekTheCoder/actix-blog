use actix_web::{error::ErrorUnauthorized, post, web::Data, Responder};

use crate::{
    services::auth::claims::{Claims, Role},
    shared::{
        db::{
            models::{admins, blogs},
            Pool,
        },
        extractors::valid_json::ValidJson,
    },
    traits::{catch_http::CatchHttp, into_response::IntoResponse},
};

use super::request::BlogCreateReq;

#[post("/")]
pub async fn create_one(
    pool: Data<Pool>,
    req: ValidJson<BlogCreateReq>,
    claims: Claims,
) -> actix_web::Result<impl Responder> {
    if claims.role != Role::Admin {
        return Err(ErrorUnauthorized("Not an admin"));
    }

    let admin = admins::by_agent_id(claims.id, pool.as_ref())
        .await
        .catch_http()?;

    let BlogCreateReq { title, content } = req.as_ref();

    let parser = pulldown_cmark::Parser::new(&content);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    blogs::create(pool.get_ref(), admin.id, title, content, &html_output)
        .await
        .into_response()
}
