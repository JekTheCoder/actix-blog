pub struct InclusiveTakeUntil<'a, I, F> {
    iter: &'a mut I,
    take_until: F,
    has_ended: bool,
}

impl<'a, I, F> InclusiveTakeUntil<'a, I, F> {
    pub fn new(iter: &'a mut I, take_until: F) -> Self {
        Self {
            iter,
            take_until,
            has_ended: false,
        }
    }
}

impl<'a, I, F> Iterator for InclusiveTakeUntil<'a, I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_ended {
            return None;
        }

        let item = self.iter.next()?;
        self.has_ended = (self.take_until)(&item);

        Some(item)
    }
}
