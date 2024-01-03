use actix_files::NamedFile;
use actix_web::{error::ErrorBadRequest, get, web::Path, Responder};
use uuid::Uuid;

use crate::domain::blog::images::{Filename, ImagePathFactory};

#[get("/{id}/public/{filename}/")]
pub async fn endpoint(
    path: Path<(Uuid, String)>,
    image_path_factory: ImagePathFactory,
) -> Result<impl Responder, actix_web::Error> {
    println!("wosi");

    let (id, filename) = path.into_inner();
    let Ok(filename) = Filename::new(&filename) else {
        return Err(ErrorBadRequest(""));
    };

    let image_path = image_path_factory.create_path(id, filename);
    println!("{}", image_path.as_ref().display());

    Ok(NamedFile::open(image_path)?)
}
