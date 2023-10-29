use actix_web::{get, web::Data, Responder};

use crate::{
    modules::{blog, db::Pool},
    shared::{extractors::partial_query::PartialQuery, models::select_slice::SelectSlice},
    sqlx::select_response,
};

#[get("/")]
pub async fn endpoint(pool: Data<Pool>, slice: PartialQuery<SelectSlice>) -> impl Responder {
    let blogs = blog::get_all(pool.get_ref(), slice.into_inner()).await;
    select_response(blogs)
}
