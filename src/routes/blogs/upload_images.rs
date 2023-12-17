use crate::modules::blog::{self, Filename, ImageManager, ImageSaveError, ALLOWED_FILETYPES};
use actix_multipart::Multipart;
use actix_web::{dev::Payload, post, web::Path, HttpResponse, Responder, ResponseError};
use futures_util::StreamExt;
use image::EncodableLayout;
use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
struct InvalidFilenameError {
    filename: String,
}

#[post("/{id}/public")]
pub async fn endpoint(
    path: Path<Uuid>,
    mut multipart: Multipart,
    image_manager: ImageManager,
) -> HttpResponse {
    let id = path.into_inner();

    let mut buffer = vec![];
    while let Some(result) = multipart.next().await {
        let Ok(mut field) = result else {
            return HttpResponse::InternalServerError().finish();
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

        // while let Some(result) = field.next().await {
        //     let Ok(bytes) = result else {
        //         return HttpResponse::InternalServerError().finish();
        //     };
        //
        //     buffer.extend_from_slice(bytes.as_bytes());
        // }

        if let Err(e) = image_manager.save(id, filename, buffer.as_ref()) {
            return match e {
                ImageSaveError::Save => HttpResponse::InternalServerError().finish(),
                ImageSaveError::Decode => HttpResponse::BadRequest().body("invalid image"),
            };
        }

        buffer.clear();
    }

    HttpResponse::Ok().finish()
}
