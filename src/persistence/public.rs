pub use config::Config;
pub use public_dir::PublicDir;

mod public_dir {
    use std::{fmt::Display, sync::Arc};

    #[repr(transparent)]
    pub struct PublicDir(str);

    impl PublicDir {
        pub fn new_arc(s: &str) -> Arc<Self> {
            let arc = Arc::<[u8]>::from(s.as_bytes());
            unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Self) }
        }
    }

    impl AsRef<str> for PublicDir {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl Display for PublicDir {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", &self.0)
        }
    }
}

mod config {
    use actix_web::web::Data;

    use crate::server::AppConfig;

    use super::public_dir::PublicDir;

    #[derive(Clone)]
    pub struct Config {
        images_dir: Data<PublicDir>,
    }

    impl Config {
        pub fn new(images_dir: Data<PublicDir>) -> Self {
            Self { images_dir }
        }
    }

    impl AppConfig for Config {
        fn configure(self, config: &mut actix_web::web::ServiceConfig) {
            config.app_data(self.images_dir);
        }
    }
}
