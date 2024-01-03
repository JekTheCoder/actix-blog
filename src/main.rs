mod shared;

mod error;
mod traits;
mod utils;

mod sqlx;

mod server;
mod domain;
mod persistence;

fn main() -> Result<(), std::io::Error> {
    <::actix_web::rt::System>::new().block_on(crate::server::run())
}
