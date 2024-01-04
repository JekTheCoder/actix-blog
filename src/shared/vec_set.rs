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
