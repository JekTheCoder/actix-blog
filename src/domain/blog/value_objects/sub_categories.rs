use uuid::Uuid;

use crate::server::shared::domain_validation::{DomainValid, Error, FieldError, FieldErrors};

pub struct SubCategories(Vec<Uuid>);

impl DomainValid for SubCategories {
    type Unchecked = Vec<Uuid>;

    fn from_unchecked(unchecked: Self::Unchecked) -> Result<Self, Error> {
        let mut errors = FieldErrors::default();

        if unchecked.is_empty() {
            errors.add(FieldError::minlen(0, 1));
        }

        if unchecked.len() > 10 {
            errors.add(FieldError::maxlen(unchecked.len(), 10));
        }

        if !errors.is_empty() {
            return Err(errors.into());
        }

        Ok(Self(unchecked))
    }
}

impl AsRef<[Uuid]> for SubCategories {
    fn as_ref(&self) -> &[Uuid] {
        &self.0
    }
}
