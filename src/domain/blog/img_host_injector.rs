use actix_web::web::Data;
use markdown_parse::ImageUrlInjector;
use uuid::Uuid;

use super::images::Filename;

use crate::{domain::server::ServerAddress, server::service::sync_service};

sync_service!(ImgHostInjectorFactory; server_address: Data<ServerAddress>);

impl Clone for ImgHostInjectorFactory {
    fn clone(&self) -> Self {
        Self {
            server_address: self.server_address.clone(),
        }
    }
}

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
    fn is_valid(&self, url: &str) -> bool {
        Filename::new(url).is_ok()
    }

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
    use super::*;
    use markdown_parse::BlogParse;

    #[test]
    fn injects_url() {
        let factory = ImgHostInjectorFactory {
            server_address: ServerAddress::new_arc("http://localhost:3000").into(),
        };

        let content = r#"# Hello my brodas
![image](wosi.jpg)"#;

        let BlogParse {
            images, content, ..
        } = markdown_parse::parse(content, &factory.create(uuid::Uuid::nil())).unwrap();

        assert!(images.into_inner().iter().any(|image| image == "wosi.jpg"));
        assert!(content.contains(
            "http://localhost:3000/blogs/00000000-0000-0000-0000-000000000000/public/wosi.jpg"
        ));
    }

    #[test]
    fn only_collects_valid_images() {
        let factory = ImgHostInjectorFactory {
            server_address: ServerAddress::new_arc("http://localhost:3000").into(),
        };

        let markdown = r#"# Hello guorld 
![image](image.png)
Hello
![bruda](./bruda.png)"#;

        let markdown_parse::BlogParse { images, .. } =
            markdown_parse::parse(markdown, &factory.create(uuid::Uuid::nil())).unwrap();

        assert_eq!(images.into_inner(), vec!["image.png".to_string()]);
    }
}
