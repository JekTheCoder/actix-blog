mod create_path;
mod save;

pub use path_factory::{ImagePathFactory};
pub use save::{save, Error as ImageSaveError};
pub use filename::Filename;

const BLOG_IMAGES_DIR: &str = "blogs";

pub const ALLOWED_FILETYPES: [mime::Mime; 2] = [mime::IMAGE_PNG, mime::IMAGE_JPEG];
const ALLOWED_MIME_NAMES: [mime::Name<'static>; 2] = [mime::PNG, mime::JPEG];


mod path_factory {
    use actix_web::{http::StatusCode, web::Data, FromRequest, ResponseError};
    use std::future::{ready, Ready};

    use crate::modules::images::ImagesDir;

    #[derive(thiserror::Error, Debug)]
    #[error("Internal error")]
    pub struct Error;

    pub struct ImagePathFactory {
        pub images_dir: Data<ImagesDir>,
    }

    impl ImagePathFactory {
        fn new_from_req(req: &actix_web::HttpRequest) -> Result<Self, Error> {
            let Some(images_dir) = req.app_data::<Data<ImagesDir>>() else {
                return Err(Error);
            };

            let image_manager = ImagePathFactory {
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

    impl FromRequest for ImagePathFactory {
        type Error = Error;
        type Future = Ready<Result<Self, Self::Error>>;

        fn from_request(
            req: &actix_web::HttpRequest,
            _: &mut actix_web::dev::Payload,
        ) -> Self::Future {
            ready(Self::new_from_req(req))
        }
    }
}

mod path {
    use std::path::{Path, PathBuf};

    use crate::modules::images;

    use super::filename::Filename;

    pub struct ImagePathBuf(images::ImagePathBuf);

    impl ImagePathBuf {
        pub fn create_ancestors(&self) -> std::io::Result<()> {
            self.0.create_ancestors()
        }
    }

    pub fn new(parent: PathBuf, filename: &Filename) -> ImagePathBuf {
        let inner = images::ImagePathBuf::new(parent, filename.as_ref());
        ImagePathBuf(inner)
    }

    impl AsRef<Path> for ImagePathBuf {
        fn as_ref(&self) -> &Path {
            self.0.as_ref()
        }
    }
}

pub mod filename {
    use super::ALLOWED_MIME_NAMES;

    use crate::modules::images;

    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Filename(images::Filename);

    impl Filename {
        pub fn new(filename: &str) -> Result<&Self, images::FilenameError> {
            let (filename, ext) = images::Filename::new_with_extension(filename)?;

            if !ALLOWED_MIME_NAMES.map(|mime| mime.as_str()).contains(&ext) {
                return Err(images::FilenameError::InvalidExtension);
            }

            Ok(unsafe { Self::unchecked_from_inner(filename) })
        }

        const unsafe fn unchecked_from_inner(filename: &images::Filename) -> &Self {
            &*(filename as *const _ as *const Self)
        }
    }

    impl AsRef<images::Filename> for Filename {
        fn as_ref(&self) -> &images::Filename {
            &self.0
        }
    }

    impl PartialEq<str> for Filename {
        fn eq(&self, other: &str) -> bool {
            &self.0 == other
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn rejects_invalid_extension() {
            let foo = "filename.not-a-valid-extension";
            assert!(Filename::new(foo).is_err());
        }

        #[test]
        fn accepts_valid_extension() {
            let foo = "filename.jpeg";
            Filename::new(foo).unwrap();
        }
    }
}
