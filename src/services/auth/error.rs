#[derive(thiserror::Error, Debug)]
#[error("Jwt encode error")]
pub struct JwtEncodeError;
