use std::{future, pin::Pin};
pub type DynFuture<T> = Pin<Box<dyn future::Future<Output = T>>>;
