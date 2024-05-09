use actix_web::web::Data;
use markdown_parse::{
    content::ContentBuf, preview::PreviewBuf, BlogParse, CowStr, ImageUrlInjector,
};
use sqlx::query;
use uuid::Uuid;

use crate::{
    domain::{
        blog::{value_objects::sub_categories::SubCategories, ImgHostInjectorFactory},
        blog_grouping,
    },
    persistence::db::Pool,
    server::service::sync_service,
};

use super::set_tags;

sync_service!(UpdateOne; pool: Data<Pool>, injector_factory: ImgHostInjectorFactory);

pub enum Error {
    Parse(markdown_parse::Error),
    NoPreview,
    Internal,
    NotFound,
}

impl From<markdown_parse::Error> for Error {
    fn from(e: markdown_parse::Error) -> Self {
        Self::Parse(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(_: sqlx::Error) -> Self {
        Self::Internal
    }
}

impl UpdateOne {
    pub async fn run(
        &self,
        id: Uuid,
        content: &ContentBuf,
        category_id: Uuid,
        preview: Option<&PreviewBuf>,
        tags: Vec<Uuid>,
        sub_categories: SubCategories,
    ) -> Result<(), Error> {
        let injector = self.injector_factory.create(id);

        let BlogParse {
            title,
            content: html_content,
            images, // TODO
        } = markdown_parse::parse(content.as_ref(), &injector)?;

        let owned_preview;
        let preview = match preview {
            Some(preview) => preview,
            None => {
                let Some(parsed_preview) = markdown_parse::parse_preview(content.as_ref()) else {
                    return Err(Error::NoPreview);
                };

                owned_preview = parsed_preview;
                &owned_preview
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
            r#"UPDATE blogs 
                SET 
                    title = $1,
                    content = $2,
                    html = $3,
                    category_id = $4,
                    preview = $5,
                    main_image = $6,
                    images = $7
                WHERE id = $8"#,
            title,
            content.as_ref(),
            &html_content,
            category_id,
            preview.as_ref(),
            main_image,
            &images,
            id,
        )
        .execute(&mut tx)
        .await?;

        if result.rows_affected() != 1 {
            return Err(Error::NotFound);
        }

        query!("DELETE FROM sub_categories_blogs WHERE blog_id = $1", id)
            .execute(&mut tx)
            .await?;

        blog_grouping::link_sub_categories(&mut tx, sub_categories.as_ref(), id).await?;

        set_tags::set_tags(&mut tx, id, tags).await?;

        tx.commit().await?;

        Ok(())
    }
}
