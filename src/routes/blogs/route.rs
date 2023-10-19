use actix_web::{
    get,
    web::{scope, Data, Path, ServiceConfig},
    HttpResponse, Responder,
};
use tokio::join;
use uuid::Uuid;

use super::{blog_response::BlogResponse, create_one::create_one};
use crate::{
    shared::{
        db::{
            models::{blogs, comments},
            Pool,
        },
        extractors::partial_query::PartialQuery,
        models::select_slice::SelectSlice,
    },
    traits::json_result::JsonResult,
};

#[get("/")]
async fn get_all(pool: Data<Pool>, slice: PartialQuery<SelectSlice>) -> impl Responder {
    blogs::get_all(pool.get_ref(), slice.into_inner())
        .await
        .json_result()
}

#[get("/{blog_id}/")]
async fn get_one(pool: Data<Pool>, id: Path<Uuid>) -> actix_web::Result<impl Responder> {
    let (blog, comments) = join!(
        blogs::by_id(pool.get_ref(), id.clone()),
        comments::by_blog(
            pool.get_ref(),
            id.into_inner(),
            SelectSlice {
                limit: 20,
                offset: 0
            }
        ),
    );
    let blog = blog?;
    let comments = comments?;

    let blog = BlogResponse {
        comments,
        id: blog.id,
        content: blog.html,
        title: blog.title,
    };

    Ok(HttpResponse::Ok().json(blog))
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/blogs")
            .service(create_one)
            .service(get_all)
            .service(get_one)
            .configure(super::comments::router),
    );
}
