use std::{
    fmt::Write,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use image::imageops::FilterType;
use uuid::Uuid;

use super::{ImageManager, BLOG_IMAGES_DIR};

const IMAGE_HEIGHT: u32 = 9 * 50;
const IMAGE_WIDTH: u32 = 16 * 50;

pub enum Error {
    Save,
    Decode,
}

impl ImageManager {
    pub fn save(&self, blog_id: Uuid, filename: &str, content: &[u8]) -> Result<(), Error> {
        let blog_images_dir = create_path(self.images_dir.as_ref().as_ref(), blog_id);

        if create_dir_all(&blog_images_dir).is_err() {
            return Err(Error::Save);
        }

        let image_path = blog_images_dir.join(filename);
        let Ok(mut image) = image::load_from_memory(content) else {
            return Err(Error::Decode);
        };

        if image.width() > IMAGE_WIDTH || image.height() > IMAGE_HEIGHT {
            image = image.resize(IMAGE_WIDTH, IMAGE_HEIGHT, FilterType::Triangle);
        }

        if image.save(&image_path).is_err() {
            return Err(Error::Save);
        }

        Ok(())
    }
}

fn create_path(images_dir: &str, blog_id: Uuid) -> PathBuf {
    let mut blog_images_dir = Path::new(images_dir).join(BLOG_IMAGES_DIR);
    blog_images_dir.push("");

    {
        let blog_images_dir = blog_images_dir.as_mut_os_string();
        blog_images_dir
            .write_fmt(format_args!("{}", blog_id))
            .unwrap();
    };

    blog_images_dir
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn creates_path_correctly() {
        let id_str = "1b9d6bcd-bbfd-4b2d-9b5d-ab8dfbbd4bed";
        let blog_id = Uuid::from_str(id_str).unwrap();

        let images_dir = "./images";

        let path = create_path(images_dir, blog_id);

        let expected = format!("{}/{}/{}", images_dir, BLOG_IMAGES_DIR, id_str);
        assert_eq!(path.as_os_str(), expected.as_str());
    }
}
