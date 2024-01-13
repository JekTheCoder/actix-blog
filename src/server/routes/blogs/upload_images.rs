use actix_multipart::Multipart;
use actix_web::{post, web::Path, HttpResponse};
use futures_util::StreamExt;
use image::EncodableLayout;
use uuid::Uuid;

use crate::domain::blog::{
    features::upload_image::{Error, UploadImage},
    images::{Filename, ImagePathFactory, ALLOWED_FILETYPES},
};

#[post("/{id}/public/")]
pub async fn endpoint(
    path: Path<Uuid>,
    mut multipart: Multipart,
    image_manager: ImagePathFactory,
    upload_image: UploadImage,
) -> HttpResponse {
    let id = path.into_inner();

    let mut buffer = vec![];
    while let Some(result) = multipart.next().await {
        let Ok(mut field) = result else {
            return HttpResponse::BadRequest().body("Could not read payload");
        };

        match field.content_type() {
            Some(filetype) => {
                if !ALLOWED_FILETYPES.contains(filetype) {
                    continue;
                }
            }
            None => continue,
        };

        let filename = field.content_disposition().get_filename().unwrap_or("");
        let Ok(filename) = Filename::new(filename) else {
            return HttpResponse::BadRequest().body(format!("invalid filename: {}", filename));
        };

        let image_path = image_manager.create_path(id, filename);

        while let Some(result) = field.next().await {
            let Ok(bytes) = result else {
                return HttpResponse::BadRequest().body("Could not read field");
            };

            buffer.extend_from_slice(bytes.as_bytes());
        }

        if let Err(e) = upload_image.run(&image_path, buffer.as_ref()) {
            return match e {
                Error::Save => HttpResponse::InternalServerError().finish(),
                Error::Decode => HttpResponse::BadRequest().body("invalid image"),
            };
        }

        buffer.clear();
    }

    HttpResponse::Ok().finish()
}
