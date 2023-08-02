use actix_web::{
    error::ErrorUnauthorized,
    get, post,
    web::{scope, Data, Path, ServiceConfig},
    HttpResponse, Responder,
};
use tokio::join;
use uuid::Uuid;

use super::{blog_request::BlogCreateReq, blog_response::BlogResponse};
use crate::{
    services::auth::claims::{Claims, Role},
    shared::{
        db::{
            models::{admins, blogs, comments},
            Pool,
        },
        extractors::{partial_query::PartialQuery, valid_json::ValidJson},
        models::select_slice::SelectSlice,
    },
    traits::{catch_http::CatchHttp, into_response::IntoResponse, json_result::JsonResult},
};

#[post("/")]
async fn create_one(
    pool: Data<Pool>,
    req: ValidJson<BlogCreateReq>,
    claims: Claims,
) -> actix_web::Result<impl Responder> {
    if claims.role != Role::Admin {
        return Err(ErrorUnauthorized("Not an admin"));
    }

    let admin = admins::by_agent_id(claims.id, pool.as_ref())
        .await
        .catch_http()?;

    let BlogCreateReq { title, content } = req.as_ref();

    blogs::create(pool.get_ref(), title, content, admin.id)
        .await
        .into_response()
}

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
