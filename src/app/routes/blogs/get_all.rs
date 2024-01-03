use actix_web::{
    get,
    web::{Data, Query},
    Responder,
};
use serde::Deserialize;

use crate::{
    app::shared::query::QuerySlice, domain::blog, persistence::db::Pool, sqlx::select_response,
};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub search: Option<String>,
    #[serde(flatten)]
    pub slice: QuerySlice,
}

#[get("/")]
pub async fn endpoint(pool: Data<Pool>, query: Query<Request>) -> impl Responder {
    let Request { search, slice } = query.into_inner();

    let blogs = blog::get_all(
        pool.get_ref(),
        slice.into(),
        search.as_deref().unwrap_or(""),
    )
    .await;
    select_response(blogs)
}
