use std::fmt::Display;

use crate::shared::str_wrapper::{buf_ops, super_str, CheckStr, buf_de};

#[derive(Debug)]
pub enum Error {
    Empty,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "can not be empty")
    }
}

#[derive(PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Content(str);

impl CheckStr for Content {
    type Error = Error;

    fn check_str(slice: &str) -> Result<(), Self::Error> {
        todo!()
    }
}

super_str!(Content);

#[derive(Debug)]
#[repr(transparent)]
pub struct ContentBuf(Box<Content>);

buf_ops!(ContentBuf, Content);
buf_de!(ContentBuf);
