pub use config::Config;
pub use images_dir::ImagesDir;

pub use filename::{Error as FilenameError, Filename};
pub use path::ImagePathBuf;

mod path {
    use std::path::{Path, PathBuf};

    use super::filename::Filename;

    /// A path type guaranteed not to be a directory
    pub struct ImagePathBuf {
        path: PathBuf,
    }

    impl ImagePathBuf {
        pub fn new(mut parent: PathBuf, filename: &Filename) -> Self {
            parent.push(filename);
            Self { path: parent }
        }

        pub fn create_ancestors(&self) -> std::io::Result<()> {
            // It's safe to unwrap because we ensure is a valid path on creation
            let ancestors = self.path.parent().unwrap();
            std::fs::create_dir_all(ancestors)
        }
    }

    impl AsRef<Path> for ImagePathBuf {
        fn as_ref(&self) -> &Path {
            &self.path
        }
    }
}

pub mod filename {
    use std::{fmt::Display, path::Path};

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Filename(str);

    impl Filename {
        pub const unsafe fn unchecked_from_str(str: &str) -> &Self {
            &*(str as *const _ as *const Self)
        }

        pub fn new_with_extension(filename: &str) -> Result<(&Self, &str), Error> {
            if filename.contains('/') {
                return Err(Error::HasParent);
            }

            let Some(at) = filename.rfind('.') else {
                return Err(Error::InvalidExtension);
            };

            if at == filename.len() - 1 {
                return Err(Error::InvalidExtension);
            }

            let ext = &filename[at + 1..];

            Ok(unsafe { (Filename::unchecked_from_str(filename), ext) })
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

    #[derive(Debug)]
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
            let filename = unsafe { Filename::unchecked_from_str(foo) };

            assert_eq!(filename, foo);
        }

        #[test]
        fn validates_extension() {
            let foo = "foo";
            let result = Filename::new_with_extension(foo);

            assert!(matches!(result, Err(Error::InvalidExtension)));
        }

        #[test]
        fn validates_extension_with_ending_dot() {
            let foo = "foo.";
            let result = Filename::new_with_extension(foo);

            assert!(matches!(result, Err(Error::InvalidExtension)));
        }

        #[test]
        fn validates_parent() {
            let foo = "foo/bar";
            let result = Filename::new_with_extension(foo);

            assert!(matches!(result, Err(Error::HasParent)));
        }
    }
}

mod images_dir {
    use std::{fmt::Display, sync::Arc};

    #[repr(transparent)]
    pub struct ImagesDir(str);

    impl ImagesDir {
        pub fn new_arc(s: &str) -> Arc<Self> {
            let arc = Arc::<[u8]>::from(s.as_bytes());
            unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Self) }
        }
    }

    impl AsRef<str> for ImagesDir {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl Display for ImagesDir {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", &self.0)
        }
    }
}

mod config {
    use actix_web::web::Data;

    use crate::app::AppConfig;

    use super::images_dir::ImagesDir;

    #[derive(Clone)]
    pub struct Config {
        images_dir: Data<ImagesDir>,
    }

    impl Config {
        pub fn new(images_dir: &str) -> Self {
            let images_dir = ImagesDir::new_arc(images_dir);
            Self {
                images_dir: Data::from(images_dir),
            }
        }
    }

    impl AppConfig for Config {
        fn configure(self, config: &mut actix_web::web::ServiceConfig) {
            config.app_data(self.images_dir);
        }
    }
}
