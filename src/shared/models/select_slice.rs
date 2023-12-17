use serde::Deserialize;

use crate::traits::partial_default::PartialDefault;

#[derive(Debug, Deserialize)]
pub struct PartialSlice {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct SelectSlice {
    pub limit: i64,
    pub offset: i64,
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
