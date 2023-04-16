pub trait PartialDefault {
    type Partial;

    fn from_partial(partial: Self::Partial) -> Self;
}
