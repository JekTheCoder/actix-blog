use actix_web::web::ServiceConfig;
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    App,
};

pub trait AppConfig {
    fn configure(self, config: &mut ServiceConfig);
}

pub trait AppConfigurable {
    fn use_config(self, config: impl AppConfig) -> Self;
}

impl<T> AppConfigurable for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn use_config(self, config: impl AppConfig) -> Self {
        self.configure(move |service_config| config.configure(service_config))
    }
}
