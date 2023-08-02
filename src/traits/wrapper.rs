pub trait Wrapper<T> {
    fn as_ref(&self) -> &T;
    fn as_mut(&mut self) -> &mut T;
    fn deref(&self) -> &T;
    fn deref_mut(&mut self) -> &mut T;
}
