use actix_web::web::Data;
use markdown_parse::{content::ContentBuf, preview::PreviewBuf};
use sqlx::query;
use uuid::Uuid;

use crate::{
    domain::{
        blog::{value_objects::sub_categories::SubCategories, ImgHostInjectorFactory},
        blog_grouping,
        user::admin_id::AdminId,
    },
    persistence::db::Pool,
    server::service::sync_service,
};

use super::set_tags;

pub use compile_content::{compile_content, compile_preview, BlogCompile};

sync_service!(CreateOne; pool: Data<Pool>, injector_factory: ImgHostInjectorFactory);

pub enum Error {
    Parse(markdown_parse::Error),
    NoPreview,
    Database,
    Conflict,
}

impl From<markdown_parse::Error> for Error {
    fn from(e: markdown_parse::Error) -> Self {
        Self::Parse(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(_) => Self::Conflict,
            _ => Self::Database,
        }
    }
}

impl CreateOne {
    pub async fn run(
        &self,
        admin_id: AdminId,
        content: &ContentBuf,
        category_id: Uuid,
        preview: Option<&PreviewBuf>,
        tags: Vec<Uuid>,
        sub_categories: SubCategories,
    ) -> Result<Uuid, Error> {
        let blog_id = Uuid::new_v4();

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

        let mut tx = self.pool.begin().await.unwrap();
        let result = query!(
            r#"INSERT INTO
        blogs(
            id,
            admin_id,
            title,
            content,
            html,
            category_id,
            preview,
            main_image,
            images
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
            blog_id,
            admin_id.into_inner(),
            title,
            content.as_ref(),
            &html_content,
            category_id,
            preview.as_str(),
            main_image,
            &images
        )
        .execute(&mut tx)
        .await?;

        if result.rows_affected() != 1 {
            return Err(Error::Conflict);
        }

        blog_grouping::link_sub_categories(&mut tx, sub_categories.as_ref(), blog_id).await?;
        set_tags::create_tags(&mut tx, blog_id, tags).await?;

        tx.commit().await?;

        Ok(blog_id)
    }
}

mod compile_content {
    use markdown_parse::{
        content::ContentBuf, preview::PreviewBuf, BlogParse, CowStr, ImageUrlInjector,
    };

    pub struct BlogCompile {
        pub title: String,
        pub html_content: String,
        pub images: Vec<String>,
        pub main_image: Option<String>,
    }

    pub fn compile_content(
        content: &ContentBuf,
        injector: impl ImageUrlInjector,
    ) -> Result<BlogCompile, markdown_parse::Error> {
        let BlogParse {
            title,
            content: html_content,
            images,
        } = markdown_parse::parse(content.as_ref(), &injector)?;

        let images = images.into_inner();
        let main_image = images.first().map(|image| {
            let mut cow = CowStr::Borrowed(image);
            injector.inject(&mut cow);

            cow.to_string()
        });

        Ok(BlogCompile {
            title,
            html_content,
            images,
            main_image,
        })
    }

    pub enum PreviewCow<'a> {
        Owned(PreviewBuf),
        Borrowed(&'a PreviewBuf),
    }

    impl PreviewCow<'_> {
        pub fn as_str(&self) -> &str {
            match self {
                Self::Owned(preview) => preview.as_ref(),
                Self::Borrowed(preview) => preview.as_ref(),
            }
        }
    }

    pub fn compile_preview<'a>(
        content: &'a ContentBuf,
        preview: Option<&'a PreviewBuf>,
    ) -> Option<PreviewCow<'a>> {
        if let Some(preview) = preview {
            return Some(PreviewCow::Borrowed(preview));
        }

        let parsed_preview = markdown_parse::parse_preview(content.as_ref())?;
        Some(PreviewCow::Owned(parsed_preview))
    }
}
