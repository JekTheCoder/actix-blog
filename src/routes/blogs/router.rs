use actix_web::{
    get, post,
    web::{scope, Data, Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use tokio::join;
use uuid::Uuid;
use validator::Validate;

use super::{
    blog::{Blog, CreateReq},
    blog_response::BlogResponse,
};
use crate::{
    db::Pool,
    extractors::{auth::AuthUser, partial_query::PartialQuery},
    models::select_slice::SelectSlice,
    routes::blogs::comments::comment::Comment,
    traits::{catch_http::CatchHttp, into_response::IntoResponse, json_result::JsonResult},
};

#[post("/")]
async fn create_one(
    pool: Data<Pool>,
    req: Json<CreateReq>,
    user: AuthUser,
) -> actix_web::Result<impl Responder> {
    req.validate().catch_http()?;
    Blog::create(pool.get_ref(), &req, user.into_inner().id)
        .await
        .into_response()
}

#[get("/")]
async fn get_all(pool: Data<Pool>, slice: PartialQuery<SelectSlice>) -> impl Responder {
    Blog::get_all(pool.get_ref(), slice.into_inner())
        .await
        .json_result()
}

#[get("/{blog_id}/")]
async fn get_one(pool: Data<Pool>, id: Path<Uuid>) -> actix_web::Result<impl Responder> {
    let (blog, comments) = join!(
        Blog::by_id(pool.get_ref(), id.clone()),
        Comment::by_blog(
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
        admin_id: blog.admin_id,
        id: blog.id,
        content: blog.content,
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
