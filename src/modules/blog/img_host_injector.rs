use std::future::{ready, Ready};

use actix_web::{web::Data, FromRequest, ResponseError};
use uuid::Uuid;

use super::parse::ImageUrlInjector;

use crate::modules::server::ServerAddress;

pub struct ImgHostInjectorFactory {
    server_address: Data<ServerAddress>,
}

impl ImgHostInjectorFactory {
    fn new(req: &actix_web::HttpRequest) -> Result<Self, Error> {
        let Some(server_address) = req.app_data::<Data<ServerAddress>>() else {
            return Err(Error);
        };

        Ok(Self {
            server_address: server_address.clone(),
        })
    }

    pub fn create(&self, blog_id: Uuid) -> ImgHostInjector {
        ImgHostInjector {
            server_address: &self.server_address,
            blog_id,
        }
    }
}

pub struct ImgHostInjector<'a> {
    server_address: &'a ServerAddress,
    blog_id: Uuid,
}

#[derive(Debug, thiserror::Error)]
#[error("")]
pub struct Error;

impl ResponseError for Error {}

impl FromRequest for ImgHostInjectorFactory {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Self::new(req))
    }
}

impl<'a> ImageUrlInjector for ImgHostInjector<'a> {
    fn inject(&self, url: &mut pulldown_cmark::CowStr<'_>) {
        let modified = format!(
            "{}/blogs/{}/public/{}",
            self.server_address, self.blog_id, url
        );
        *url = modified.into();
    }
}
