use crate::{
    domain::{auth::Claims, comment},
    persistence::db::Pool,
    shared::extractors::valid_json::ValidJson,
    sqlx::insert_response,
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
    req: ValidJson<comment::CreateComment>,
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
