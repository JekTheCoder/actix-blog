use image::imageops::FilterType;

use super::path::ImagePathBuf;

const IMAGE_HEIGHT: u32 = 9 * 50;
const IMAGE_WIDTH: u32 = 16 * 50;

pub enum Error {
    Save,
    Decode,
}

pub fn save(image_path: ImagePathBuf, content: &[u8]) -> Result<(), Error> {
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
