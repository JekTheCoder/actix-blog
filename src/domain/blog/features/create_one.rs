use actix_web::web::Data;
use pulldown_cmark::CowStr;
use sqlx::query;
use uuid::Uuid;

use crate::{
    domain::{
        blog::{
            self, parse_preview, value_objects::{preview::Preview, content::Content, sub_categories::SubCategories}, BlogParse, ImageUrlInjector,
            ImgHostInjectorFactory,
        },
        category,
        user::value_objects::AdminId,
    },
    persistence::db::Pool,
    server::service::sync_service,
};

sync_service!(CreateOne; pool: Data<Pool>, injector_factory: ImgHostInjectorFactory);

pub enum Error {
    Parse(blog::ParseError),
    NoPreview,
    Database,
    Conflict,
}

impl From<blog::parse::Error> for Error {
    fn from(e: blog::parse::Error) -> Self {
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
        content: &Content,
        category_id: Uuid,
        preview: Option<&Preview>,
        tags: Vec<Uuid>,
        sub_categories: SubCategories,
    ) -> Result<Uuid, Error> {
        let blog_id = Uuid::new_v4();

        let injector = self.injector_factory.create(blog_id);

        let BlogParse {
            title,
            content: html_content,
            images, // TODO
        } = blog::parse(content.as_ref(), &injector)?;

        let owned_preview;
        let preview = match preview {
            Some(preview) => preview,
            None => {
                let Some(parsed_preview) = parse_preview(content.as_ref()) else {
                    return Err(Error::NoPreview);
                };

                owned_preview = parsed_preview;
                owned_preview.as_ref()
            }
        };

        let images = images.into_inner();
        let main_image = images.first().map(|image| {
            let mut cow = CowStr::Borrowed(image);
            injector.inject(&mut cow);

            cow.to_string()
        });

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
            preview.as_ref(),
            main_image,
            &images
        )
        .execute(&mut tx)
        .await?;

        if result.rows_affected() != 1 {
            return Err(Error::Conflict);
        }

        category::link_sub_categories(&mut tx, sub_categories.as_ref(), blog_id).await?;
        category::link_tags(&mut tx, tags, blog_id).await?;

        tx.commit().await?;

        Ok(blog_id)
    }
}
