mod domain;
mod persistence;
mod server;
mod shared;

fn main() -> Result<(), std::io::Error> {
    <::actix_web::rt::System>::new().block_on(crate::server::run())
}
