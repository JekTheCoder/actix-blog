use std::{
    fmt::Write,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use image::imageops::FilterType;
use uuid::Uuid;

use super::{ImageManager, BLOG_IMAGES_DIR, filename::Filename, path::ImagePathBuf};

const IMAGE_HEIGHT: u32 = 9 * 50;
const IMAGE_WIDTH: u32 = 16 * 50;

pub enum Error {
    Save,
    Decode,
}

impl ImageManager {
    pub fn save(&self, image_path: ImagePathBuf, content: &[u8]) -> Result<(), Error> {
        if image_path.create_ancestors().is_err() {
            return Err(Error::Save);
        }

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
