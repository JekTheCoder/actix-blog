use actix_web::web::Data;
use markdown_parse::{content::ContentBuf, preview::PreviewBuf};
use sqlx::query;
use uuid::Uuid;

use crate::{
    domain::blog::ImgHostInjectorFactory, persistence::db::Pool, server::service::sync_service,
};

use super::create_one::{compile_content, compile_preview, BlogCompile};

sync_service!(SetContent; pool: Data<Pool>, injector_factory: ImgHostInjectorFactory);

pub enum Error {
    Parse(markdown_parse::Error),
    NoPreview,
    Database,
}

impl From<markdown_parse::Error> for Error {
    fn from(e: markdown_parse::Error) -> Self {
        Self::Parse(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Database
    }
}

impl SetContent {
    pub async fn run(
        self,
        blog_id: Uuid,
        content: &ContentBuf,
        preview: Option<&PreviewBuf>,
    ) -> Result<(), Error> {
        let injector = self.injector_factory.create(blog_id);
        let BlogCompile {
            title,
            html_content,
            images,
            main_image,
        } = compile_content(content, injector)?;

        let Some(preview) = compile_preview(content, preview) else {
            return Err(Error::NoPreview);
        };

        let _ = query!(
            "UPDATE blogs SET content = $1, html = $2, preview = $3, main_image = $4, images = $5 WHERE id = $6",
            content.as_ref(),
            html_content,
            preview.as_str(),
            main_image,
            images.as_slice(),
            blog_id
        )
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }
}
