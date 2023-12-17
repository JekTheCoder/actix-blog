use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder, ResponseError,
};
use tokio::join;
use uuid::Uuid;

use crate::{modules::{blog, db::Pool, comment}, shared::models::select_slice::SelectSlice};

use super::comments::response::CommentByBlog;

#[derive(serde::Serialize)]
pub struct Response {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub comments: Vec<CommentByBlog>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Blog not found")]
    NotFound,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::NotFound => HttpResponse::NotFound().finish(),
        }
    }
}

#[get("/{blog_id}/")]
pub async fn endpoint(pool: Data<Pool>, id: Path<Uuid>) -> Result<impl Responder, Error> {
    let (blog, comments) = join!(
        blog::by_id(pool.get_ref(), *id),
        comment::by_blog(
            pool.get_ref(),
            id.into_inner(),
            SelectSlice {
                limit: 20,
                offset: 0
            }
        ),
    );

    let blog = blog.map_err(|_| Error::NotFound)?;
    let comments = comments.unwrap_or_else(|_| vec![]);

    let blog = Response {
        comments: comments.into_iter().map(Into::into).collect(),
        id: blog.id,
        content: blog.html,
        title: blog.title,
    };

    Ok(HttpResponse::Ok().json(blog))
}
