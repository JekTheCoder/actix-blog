pub use address::Address as ServerAddress;
pub use config::Config;

mod address {
    use std::{fmt::Display, sync::Arc};

    #[repr(transparent)]
    pub struct Address(str);

    impl Address {
        pub fn new_arc(s: &str) -> Arc<Self> {
            let arc = Arc::<[u8]>::from(s.as_bytes());
            unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Self) }
        }
    }

    impl AsRef<str> for Address {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl Display for Address {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }
}

mod config {
    use actix_web::web::{Data, ServiceConfig};

    use crate::server::AppConfig;

    use super::address::Address;

    #[derive(Clone)]
    pub struct Config {
        address: Data<Address>,
    }

    impl Config {
        pub fn new(address: &str) -> Self {
            let protocol = Address::new_arc(address);
            Self {
                address: Data::from(protocol),
            }
        }
    }

    impl AppConfig for Config {
        fn configure(self, config: &mut ServiceConfig) {
            config.app_data(self.address);
        }
    }
}
