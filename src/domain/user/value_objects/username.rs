use crate::{
    server::shared::domain_validation::{self, DomainValid, FieldError},
    shared::str_wrapper::{buf_ops, super_str, CheckStr},
};

#[derive(Debug)]
pub enum Error {
    Empty,
    Maxlen,
    InvalidChar,
}

const MAX_LEN: usize = 100;

fn username_invalid_char(c: char) -> bool {
    !c.is_ascii()
        || c.is_whitespace()
        || c.is_ascii_punctuation()
        || c.is_ascii_control()
        || c == '@'
}

#[derive(PartialEq, Eq, Debug, serde::Serialize)]
#[repr(transparent)]
pub struct Username(str);

impl CheckStr for Username {
    type Error = Error;

    fn check_str(slice: &str) -> Result<(), Self::Error> {
        if slice.is_empty() {
            return Err(Error::Empty);
        }

        if slice.len() > MAX_LEN {
            return Err(Error::Maxlen);
        }

        if slice.contains(username_invalid_char) {
            return Err(Error::InvalidChar);
        }

        Ok(())
    }
}

super_str!(Username);

#[derive(Debug, serde::Serialize)]
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

        if unchecked.len() > MAX_LEN {
            errors.add(FieldError::maxlen(unchecked.len(), 100));
        }

        if unchecked.contains(username_invalid_char) {
            errors.add(FieldError::custom("invalid chars"));
        }

        if errors.is_empty() {
            let boxed: Box<str> = unchecked.into();
            Ok(Self::from_boxed_unchecked(boxed))
        } else {
            Err(domain_validation::Error::Field(errors))
        }
    }
}
