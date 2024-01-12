use actix_web::{get, web::Query, Responder};
use serde::Deserialize;

use crate::{
    domain::blog::features::get_all::GetAll,
    server::shared::{query::QuerySlice, response::select_response},
};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub search: Option<String>,
    #[serde(flatten)]
    pub slice: QuerySlice,
}

#[get("/")]
pub async fn endpoint(get_all: GetAll, query: Query<Request>) -> impl Responder {
    let Request { search, slice } = query.into_inner();

    let blogs = get_all.run(slice, search.as_deref().unwrap_or("")).await;

    select_response(blogs)
}
