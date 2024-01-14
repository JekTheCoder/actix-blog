use actix_web::{put, web::Json, HttpResponse, Responder};

use crate::{
    domain::user::change_password,
    server::auth::{Claims, PasswordHasher},
};

#[derive(serde::Deserialize)]
pub struct Request {
    new_password: String,
    old_password: String,
}

#[put("/password")]
pub async fn endpoint(
    req: Json<Request>,
    password_hasher: PasswordHasher,
    claims: Claims,
    change_password: change_password::ChangePassword,
) -> impl Responder {
    let account_id = claims.id;

    let Request {
        new_password,
        old_password,
    } = req.into_inner();

    let hashed_password = password_hasher.hash(new_password);

    match change_password
        .run(account_id, &hashed_password, &old_password)
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(change_password::Error::NotEqual | change_password::Error::NotFound) => {
            HttpResponse::Unauthorized().finish()
        }
        Err(change_password::Error::Internal) => HttpResponse::InternalServerError().finish(),
    }
}
