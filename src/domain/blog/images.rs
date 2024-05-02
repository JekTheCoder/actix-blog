mod create_path;

use crate::{persistence::images::ImagesDir, server::service::sync_service};
use actix_web::web::Data;

pub use filename::Filename;
pub use path::ImagePathBuf;

const BLOG_IMAGES_DIR: &str = "blogs";

pub const ALLOWED_FILETYPES: [mime::Mime; 2] = [mime::IMAGE_PNG, mime::IMAGE_JPEG];
const ALLOWED_MIME_NAMES: [&'static str; 3] = ["png", "jpg", "jpeg"];

sync_service!(ImagePathFactory; images_dir: Data<ImagesDir>);

mod path {
    use std::path::{Path, PathBuf};

    use crate::persistence::images;

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

    use crate::persistence::images;

    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Filename(images::Filename);

    impl Filename {
        pub fn new(filename: &str) -> Result<&Self, images::FilenameError> {
            let (filename, ext) = images::Filename::new_with_extension(filename)?;

            if !ALLOWED_MIME_NAMES.contains(&ext) {
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
