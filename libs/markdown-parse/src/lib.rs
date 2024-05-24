mod component_parse;
mod value_objects;
mod parse;

pub use parse::{parse, parse_preview, Error, ImageUrlInjector, BlogParse, PreviewParse};
pub use value_objects::{content, preview};
pub use pulldown_cmark::CowStr;

mod vec_set {
    #[derive(Debug, Default)]
    pub struct VecSet<T: Eq>(Vec<T>);

    impl<T> VecSet<T>
    where
        T: Eq,
    {
        pub fn insert(&mut self, item: T) -> bool {
            let contains = self.0.contains(&item);
            if !contains {
                self.0.push(item);
            }

            !contains
        }

        pub fn into_inner(self) -> Vec<T> {
            self.0
        }
    }
}

