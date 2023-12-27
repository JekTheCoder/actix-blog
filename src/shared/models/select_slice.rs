use serde::Deserialize;

use crate::traits::partial_default::PartialDefault;

use super::flatten_slice::FlattenSlice;

#[derive(Debug, Deserialize)]
pub struct PartialSlice {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct SelectSlice {
    pub limit: u32,
    pub offset: u32,
}

impl Default for SelectSlice {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
        }
    }
}

impl PartialDefault for SelectSlice {
    type Partial = PartialSlice;

    fn from_partial(partial: Self::Partial) -> Self {
        let Self::Partial { limit, offset } = partial;

        Self {
            limit: limit.unwrap_or(20),
            offset: offset.unwrap_or(0),
        }
    }
}

impl SelectSlice {
    pub const fn from_flatten(slice: FlattenSlice) -> Self {
        Self {
            limit: slice.limit,
            offset: slice.offset,
        }
    }
}
