use actix_web::web::Data;
use markdown_parse::{content::ContentBuf, preview::PreviewBuf};
use sqlx::query;
use uuid::Uuid;

use crate::{
    domain::blog::ImgHostInjectorFactory, persistence::db::Pool, server::service::sync_service,
};

use super::create_one::{compile_content, BlogCompile};

sync_service!(SetContent; pool: Data<Pool>, injector_factory: ImgHostInjectorFactory);

impl Clone for SetContent {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            injector_factory: self.injector_factory.clone(),
        }
    }
}

#[derive(Debug)]
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
    fn from(_: sqlx::Error) -> Self {
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

        let markdown_parse::PreviewParse {
            preview,
            description,
        } = {
            let preview_markdown = preview
                .map(|preview| preview.as_ref())
                .unwrap_or_else(|| content.as_ref());

            match markdown_parse::parse_preview(preview_markdown) {
                Some(preview) => preview,
                None => return Err(Error::NoPreview),
            }
        };

        let _ = query!(
            "UPDATE blogs SET title = $1, content = $2, html = $3, preview = $4, description = $5, main_image = $6, images = $7 WHERE id = $8",
            title,
            content.as_ref(),
            html_content,
            preview.as_str(),
            description,
            main_image,
            images.as_slice(),
            blog_id
        )
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }
}
