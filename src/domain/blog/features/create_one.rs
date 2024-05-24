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

pub use compile_content::{compile_content, BlogCompile};

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
            description,
            main_image,
            images
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
            blog_id,
            admin_id.into_inner(),
            title,
            content.as_ref(),
            &html_content,
            category_id,
            preview.as_str(),
            description,
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
    use markdown_parse::{content::ContentBuf, BlogParse, CowStr, ImageUrlInjector};

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
}
