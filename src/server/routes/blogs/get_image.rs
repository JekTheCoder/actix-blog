use actix_files::NamedFile;
use actix_web::{error::ErrorBadRequest, get, web::Path, Responder};
use uuid::Uuid;

use crate::domain::blog::{features::get_image::GetImage, images::Filename};

#[get("/{id}/public/{filename}/")]
pub async fn endpoint(
    path: Path<(Uuid, String)>,
    get_image: GetImage,
) -> Result<impl Responder, actix_web::Error> {
    let (id, filename) = path.into_inner();

    let Ok(filename) = Filename::new(&filename) else {
        return Err(ErrorBadRequest(""));
    };

    let image_path = get_image.run(id, filename);

    Ok(NamedFile::open(image_path)?)
}
