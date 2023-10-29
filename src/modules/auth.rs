mod error;
mod models;
mod services;
mod utils;

use actix_web::web::{Data, ServiceConfig};

pub use services::auth_decoder::AuthDecoder;
pub use services::auth_encoder::AuthEncoder;
pub use services::refresh_decoder::RefreshDecoder;

pub use models::{claims::Claims, claims_data::ClaimsData, role::Role, tokens::Tokens};

pub fn configure(app: &mut ServiceConfig) {
    app.app_data(Data::new(AuthDecoder::default()))
        .app_data(Data::new(RefreshDecoder::default()))
        .app_data(Data::new(AuthEncoder::default()));
}
