use actix_web::{
    put,
    web::{Json, Path},
    Responder,
};
use uuid::Uuid;

use crate::{
    domain::blog::features::set_tags::SetTags,
    server::{admin::IsAdminFactory, shared::response::insert_response},
};

#[put("/", wrap = "IsAdminFactory")]
pub async fn endpoint(
    blog_id: Path<Uuid>,
    req: Json<Vec<Uuid>>,
    set_tags: SetTags,
) -> impl Responder {
    let tags = req.into_inner();

    let response = set_tags.run(blog_id.into_inner(), tags).await;
    insert_response(response)
}
