mod save;

use std::future::{ready, Ready};

use actix_web::{http::StatusCode, web::Data, FromRequest, ResponseError};

use crate::modules::images::ImagesDir;

const BLOG_IMAGES_DIR: &str = "blogs";

#[derive(thiserror::Error, Debug)]
#[error("Internal error")]
pub struct Error;

pub struct ImageManager {
    pub images_dir: Data<ImagesDir>,
}

impl ImageManager {
    fn new_from_req(req: &actix_web::HttpRequest) -> Result<Self, Error> {
        let Some(images_dir) = req.app_data::<Data<ImagesDir>>() else {
            return Err(Error);
        };

        let image_manager = ImageManager {
            images_dir: images_dir.clone(),
        };
        Ok(image_manager)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl FromRequest for ImageManager {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Self::new_from_req(req))
    }
}
