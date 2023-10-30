use std::iter::Peekable;

pub struct PeekTakeUntil<'a, I, F> {
    iter: &'a mut I,
    take_until: F,
}

impl<'a, I, F> PeekTakeUntil<'a, I, F> {
    pub fn new(iter: &'a mut I, take_until: F) -> Self {
        Self { iter, take_until }
    }
}

impl<'a, I, F> Iterator for PeekTakeUntil<'a, Peekable<I>, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.peek()?;
        let has_ended = (self.take_until)(item);

        if has_ended {
            None
        } else {
            self.iter.next()
        }
    }
}
