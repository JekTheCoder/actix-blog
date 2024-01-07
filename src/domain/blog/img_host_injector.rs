use actix_web::web::Data;
use uuid::Uuid;

use super::parse::ImageUrlInjector;

use crate::{domain::server::ServerAddress, server::service::sync_service};

sync_service!(ImgHostInjectorFactory; server_address: Data<ServerAddress>);

impl ImgHostInjectorFactory {
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

impl<'a> ImageUrlInjector for ImgHostInjector<'a> {
    fn inject(&self, url: &mut pulldown_cmark::CowStr<'_>) {
        let modified = format!(
            "{}/blogs/{}/public/{}",
            self.server_address, self.blog_id, url
        );
        *url = modified.into();
    }
}
