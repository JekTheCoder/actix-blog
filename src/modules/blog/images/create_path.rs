use std::path::Path;

use super::{
    filename::Filename,
    path::{new, ImagePathBuf},
    ImageManager, BLOG_IMAGES_DIR,
};

impl ImageManager {
    pub fn create_path(&self, filename: &Filename) -> ImagePathBuf {
        let dir = Path::new(self.images_dir.as_ref().as_ref()).join(BLOG_IMAGES_DIR);
        new(dir, filename)
    }
}
