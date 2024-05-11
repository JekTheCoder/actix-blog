pub mod images;
mod img_host_injector;

pub mod features;
pub mod value_objects;

pub use config::Config;
pub use img_host_injector::ImgHostInjectorFactory;

mod config {
    use std::path::PathBuf;

    use actix_web::web::Data;

    use crate::{persistence::public::PkgDir, server::AppConfig};

    #[derive(Clone)]
    pub struct Config {
        pkg_dir: Data<PkgDir>,
    }

    impl Config {
        pub fn new(pkg_dir: Data<PkgDir>) -> Self {
            Self { pkg_dir }
        }
    }

    impl AppConfig for Config {
        fn configure(self, config: &mut actix_web::web::ServiceConfig) {
            config.service(actix_files::Files::new(
                "/blogs/pkg/",
                PathBuf::from(self.pkg_dir.as_ref()).join("blogs"),
            ));
        }
    }
}
