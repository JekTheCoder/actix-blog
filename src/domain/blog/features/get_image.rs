use uuid::Uuid;

use crate::{
    domain::blog::images::{Filename, ImagePathBuf, ImagePathFactory},
    server::service::sync_service,
};

sync_service!(GetImage; img_path_factory: ImagePathFactory);

impl GetImage {
    pub fn run(&self, id: Uuid, filename: &Filename) -> ImagePathBuf {
        self.img_path_factory.create_path(id, filename)
    }
}
