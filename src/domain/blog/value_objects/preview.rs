use std::fmt::Display;

use crate::{
    server::shared::domain_validation::{self, DomainValid, FieldError},
    shared::str_wrapper::{buf_ops, super_str, CheckStr},
};

#[derive(Clone, Debug)]
pub enum Error {
    Empty,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "can not be empty"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Preview(str);

impl CheckStr for Preview {
    type Error = Error;

    fn check_str(slice: &str) -> Result<(), Self::Error> {
        if slice.is_empty() {
            return Err(Error::Empty);
        }

        Ok(())
    }
}

super_str!(Preview);

#[derive(Debug)]
pub struct PreviewBuf(Box<Preview>);

buf_ops!(PreviewBuf, Preview);

impl DomainValid for PreviewBuf {
    type Unchecked = String;

    fn from_unchecked(
        unchecked: Self::Unchecked,
    ) -> Result<Self, crate::server::shared::domain_validation::error::Error> {
        let unchecked = unchecked.trim();
        let mut errors = domain_validation::FieldErrors::default();

        if unchecked.is_empty() {
            errors.add(FieldError::minlen(unchecked.len(), 1));
        }

        if unchecked.len() > 400 {
            errors.add(FieldError::maxlen(unchecked.len(), 400));
        }

        if errors.is_empty() {
            let boxed: Box<str> = unchecked.into();
            Ok(Self::from_boxed_unchecked(boxed))
        } else {
            Err(domain_validation::Error::Field(errors))
        }
    }
}
