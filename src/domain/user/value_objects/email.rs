use validator::validate_email;

use crate::{
    server::shared::domain_validation::{self, DomainValid, FieldError},
    shared::str_wrapper::{buf_ops, super_str, CheckStr},
};

pub enum Error {}

#[derive(PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Email(str);

impl CheckStr for Email {
    type Error = Error;

    fn check_str(slice: &str) -> Result<(), Self::Error> {
        todo!()
    }
}

super_str!(Email);

#[derive(Debug)]
pub struct EmailBuf(Box<Email>);

buf_ops!(EmailBuf, Email);

impl DomainValid for EmailBuf {
    type Unchecked = String;

    fn from_unchecked(
        unchecked: Self::Unchecked,
    ) -> Result<Self, crate::server::shared::domain_validation::error::Error> {
        let mut errors = domain_validation::FieldErrors::default();

        if !validate_email(&unchecked) {
            errors.add(FieldError::email());
        }

        if errors.is_empty() {
            let boxed: Box<str> = unchecked.into();
            Ok(Self::from_boxed_unchecked(boxed))
        } else {
            Err(domain_validation::Error::Field(errors))
        }
    }
}
