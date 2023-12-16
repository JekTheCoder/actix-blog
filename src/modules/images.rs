pub use images_dir::ImagesDir;
pub use config::Config;

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

    use crate::actix::AppConfig;

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
