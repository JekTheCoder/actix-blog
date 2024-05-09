pub mod images;
mod img_host_injector;

pub mod features;
pub mod value_objects;

pub use config::Config;
pub use img_host_injector::ImgHostInjectorFactory;

mod config {
    use std::path::PathBuf;

    use actix_web::web::Data;

    use crate::{persistence::public::PublicDir, server::AppConfig};

    #[derive(Clone)]
    pub struct Config {
        public_dir: Data<PublicDir>,
    }

    impl Config {
        pub fn new(public_dir: Data<PublicDir>) -> Self {
            Self { public_dir }
        }
    }

    impl AppConfig for Config {
        fn configure(self, config: &mut actix_web::web::ServiceConfig) {
            config.service(actix_files::Files::new(
                "/blogs/pkg/",
                PathBuf::from(self.public_dir.as_ref().as_ref()).join("pkg"),
            ));
        }
    }
}
