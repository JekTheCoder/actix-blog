use std::fmt::Display;

use crate::shared::str_wrapper::{buf_ops, super_str, CheckStr, buf_de};

#[derive(Clone, Debug)]
pub enum Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Preview(str);

impl CheckStr for Preview {
    type Error = Error;

    fn check_str(slice: &str) -> Result<(), Self::Error> {
        todo!()
    }
}

super_str!(Preview);

#[derive(Debug)]
pub struct PreviewBuf(Box<Preview>);

buf_ops!(PreviewBuf, Preview);
buf_de!(PreviewBuf);
