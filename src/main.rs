mod error;
mod traits;
mod utils;

mod server;
mod domain;
mod persistence;

fn main() -> Result<(), std::io::Error> {
    <::actix_web::rt::System>::new().block_on(crate::server::run())
}
