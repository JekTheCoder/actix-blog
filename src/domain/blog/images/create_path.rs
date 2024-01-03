use std::{
    fmt::Write,
    path::{Path, PathBuf},
};

use uuid::Uuid;

use super::{
    filename::Filename,
    path::{new, ImagePathBuf},
    ImagePathFactory, BLOG_IMAGES_DIR,
};

impl ImagePathFactory {
    pub fn create_path(&self, blog_id: Uuid, filename: &Filename) -> ImagePathBuf {
        let dir_path = create_dir_path(self.images_dir.as_ref().as_ref(), blog_id);
        new(dir_path, filename)
    }
}

fn create_dir_path(images_dir: &str, blog_id: Uuid) -> PathBuf {
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

        let path = create_dir_path(images_dir, blog_id);

        let expected = format!("{}/{}/{}", images_dir, BLOG_IMAGES_DIR, id_str);
        assert_eq!(path.as_os_str(), expected.as_str());
    }
}
