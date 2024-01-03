use crate::{
    domain::comment,
    persistence::db::Pool,
    server::shared::{query::QuerySlice, response::select_response},
};

use super::response::CommentByBlog;

use actix_web::{
    get,
    web::{Data, Path, Query},
    Responder,
};
use uuid::Uuid;

#[get("/")]
pub async fn endpoint(
    pool: Data<Pool>,
    blog_id: Path<Uuid>,
    slice: Query<QuerySlice>,
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
