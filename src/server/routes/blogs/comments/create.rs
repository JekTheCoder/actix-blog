use crate::{
    domain::comment,
    persistence::db::Pool,
    server::auth::Claims,
    server::shared::{query::ValidJson, response::insert_response},
};

use actix_web::{
    post,
    web::{Data, Path},
    Responder,
};
use uuid::Uuid;

#[post("/")]
pub async fn endpoint(
    pool: Data<Pool>,
    blog_id: Path<Uuid>,
    claims: Claims,
    req: ValidJson<comment::models::CreateComment>,
) -> impl Responder {
    let result = comment::create(
        pool.get_ref(),
        req.as_ref(),
        claims.id,
        blog_id.into_inner(),
    )
    .await;

    insert_response(result)
}
