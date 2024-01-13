use crate::{
    server::shared::domain_validation::{self, DomainValid, FieldError},
    shared::str_wrapper::{buf_ops, super_str, CheckStr},
};

pub enum Error {
    Empty,
    Maxlen,
}

#[derive(PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Username(str);

impl CheckStr for Username {
    type Error = Error;

    fn check_str(slice: &str) -> Result<(), Self::Error> {
        if slice.is_empty() {
            return Err(Error::Empty);
        }

        Ok(())
    }
}

super_str!(Username);

#[derive(Debug)]
pub struct UsernameBuf(Box<Username>);

buf_ops!(UsernameBuf, Username);

impl DomainValid for UsernameBuf {
    type Unchecked = String;

    fn from_unchecked(
        unchecked: Self::Unchecked,
    ) -> Result<Self, crate::server::shared::domain_validation::error::Error> {
        let unchecked = unchecked.trim();
        let mut errors = domain_validation::FieldErrors::default();

        if unchecked.is_empty() {
            errors.add(FieldError::minlen(unchecked.len(), 1));
        }

        if unchecked.len() > 100 {
            errors.add(FieldError::maxlen(unchecked.len(), 100));
        }

        if errors.is_empty() {
            let boxed: Box<str> = unchecked.into();
            Ok(Self::from_boxed_unchecked(boxed))
        } else {
            Err(domain_validation::Error::Field(errors))
        }
    }
}
