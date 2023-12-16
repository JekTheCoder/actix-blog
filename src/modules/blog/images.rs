mod save;

use std::future::{ready, Ready};

use actix_web::{http::StatusCode, web::Data, FromRequest, ResponseError};

use crate::modules::images::ImagesDir;

const BLOG_IMAGES_DIR: &str = "blogs";

pub const ALLOWED_FILETYPES: [mime::Mime; 2] = [mime::IMAGE_PNG, mime::IMAGE_JPEG];
const ALLOWED_MIME_NAMES: [mime::Name<'static>; 2] = [mime::PNG, mime::JPEG];

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

mod filename {
    use std::{fmt::Display, path::Path};

    use super::ALLOWED_MIME_NAMES;

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Filename(str);

    impl Filename {
        pub unsafe fn unchecked_from_str(str: &str) -> &Self {
            &*(str as *const _ as *const Self)
        }

        pub fn new(filename: &str) -> Result<&Self, Error> {
            if filename.contains('/') {
                return Err(Error::HasParent);
            }

            if !filename.rfind('.').is_some_and(|at| {
                let ext = &filename[at + 1..];
                ALLOWED_MIME_NAMES.iter().any(|mime| mime.as_str() == ext)
            }) {
                return Err(Error::InvalidExtension);
            }

            Ok(unsafe { Filename::unchecked_from_str(filename) })
        }
    }

    impl AsRef<Path> for Filename {
        fn as_ref(&self) -> &Path {
            Path::new(&self.0)
        }
    }

    impl AsRef<str> for Filename {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl Display for Filename {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    pub enum Error {
        HasParent,
        InvalidExtension,
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
        fn can_cast_filename() {
            let foo = "foo";
            let filename = unsafe { Filename::unchecked_from_str(&foo) };

            assert_eq!(filename, foo);
        }

        #[test]
        fn validates_extension() {
            let foo = "foo";
            let filename = Filename::new(foo);

            assert!(matches!(filename, Err(Error::InvalidExtension)));
        }

        #[test]
        fn validates_extension_with_ending_dot() {
            let foo = "foo.";
            let filename = Filename::new(foo);

            assert!(matches!(filename, Err(Error::InvalidExtension)));
        }

        #[test]
        fn validates_parent() {
            let foo = "foo/bar";
            let filename = Filename::new(foo);

            assert!(matches!(filename, Err(Error::HasParent)));
        }
    }
}
