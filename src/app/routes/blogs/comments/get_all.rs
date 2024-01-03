use crate::{
    domain::comment,
    persistence::db::Pool,
    shared::{extractors::partial_query::PartialQuery, models::select_slice::SelectSlice},
    sqlx::select_response,
};

use super::response::CommentByBlog;

use actix_web::{
    get,
    web::{Data, Path},
    Responder,
};
use uuid::Uuid;

#[get("/")]
pub async fn endpoint(
    pool: Data<Pool>,
    blog_id: Path<Uuid>,
    slice: PartialQuery<SelectSlice>,
) -> impl Responder {
    let result = comment::by_blog(
        pool.get_ref(),
        blog_id.into_inner(),
        slice.into_inner().into(),
    )
    .await
    .map(|comments| {
        comments
            .into_iter()
            .map(Into::into)
            .collect::<Vec<CommentByBlog>>()
    });

    select_response(result)
}
