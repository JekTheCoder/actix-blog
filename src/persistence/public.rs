pub use config::Config;
pub use public_dir::PublicDir;
pub use pkg_dir::PkgDir;

macro_rules! impl_dir {
    ($name: ident) => {
        #[repr(transparent)]
        pub struct $name(str);

        impl $name {
            pub fn new_arc(s: &str) -> std::sync::Arc<Self> {
                let arc = std::sync::Arc::<[u8]>::from(s.as_bytes());
                unsafe { std::sync::Arc::from_raw(std::sync::Arc::into_raw(arc) as *const Self) }
            }

            pub fn new_data(s: &str) -> actix_web::web::Data<Self> {
                actix_web::web::Data::from($name::new_arc(s))
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<std::path::Path> for $name {
            fn as_ref(&self) -> &std::path::Path {
                std::path::Path::new(&self.0)
            }
        }

        impl<'a> From<&'a $name> for std::path::PathBuf {
            fn from(s: &'a $name) -> Self {
                std::path::PathBuf::from(&s.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.0)
            }
        }
    };
}

use impl_dir;

mod pkg_dir {
    use super::impl_dir;

    impl_dir!(PkgDir);
}

mod public_dir {
    use super::impl_dir;

    impl_dir!(PublicDir);
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
