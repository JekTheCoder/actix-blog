use std::{future, pin::Pin};
pub type Future<T> = Pin<Box<dyn future::Future<Output = T>>>;
