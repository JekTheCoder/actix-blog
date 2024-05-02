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

#[cfg(test)]
mod tests {
    use crate::domain::blog::BlogParse;

    use super::*;

    #[test]
    fn injects_url() {
        let factory = ImgHostInjectorFactory {
            server_address: ServerAddress::new_arc("http://localhost:3000").into(),
        };

        let content = r#"# Hello my brodas
![image](wosi.jpg)"#;

        let BlogParse {
            images, content, ..
        } = crate::domain::blog::parse(content, &factory.create(uuid::Uuid::nil())).unwrap();

        assert!(images.into_inner().iter().any(|image| image == "wosi.jpg"));
        assert!(content.contains(
            "http://localhost:3000/blogs/00000000-0000-0000-0000-000000000000/public/wosi.jpg"
        ));
    }
}
