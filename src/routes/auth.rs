use actix_web::{
    post,
    web::{self, scope, Data, Json, ServiceConfig},
    HttpResponse, Responder, ResponseError,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use validator::Validate;

use crate::{
    app::AppData,
    models::user::{self, CreateReq, User},
    services::auth::{AuthEncoder, JwtEncodeError},
    traits::into_http::IntoHttp,
};

#[derive(Clone, Debug, Deserialize)]
struct LoginReq {
    username: String,
    password: String,
}

#[derive(Debug, thiserror::Error)]
#[error("username or password invalid")]
struct LoginInvalid;

impl ResponseError for LoginInvalid {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }
}

#[derive(Serialize)]
struct LoginResponse {
    user: user::Response,
    token: String,
    refresh_token: String,
}

#[derive(Serialize)]
struct Tokens {
    token: String,
    refresh_token: String,
}

fn authorize(encoder: &AuthEncoder, id: uuid::Uuid) -> Result<Tokens, JwtEncodeError> {
    let auth_token = encoder.auth(id)?;
    let refresh_token = encoder.refresh(id)?;

    Ok(Tokens {
        token: auth_token,
        refresh_token,
    })
}

#[post("/login/")]
async fn login(
    app: AppData,
    encoder: Data<AuthEncoder>,
    req: Json<LoginReq>,
) -> actix_web::Result<impl Responder> {
    let LoginReq { username, password } = req.0;

    let found = query_as!(
        user::User,
        "SELECT * FROM users WHERE username = $1",
        username
    )
    .fetch_one(&app.pool)
    .await
    .map_err(|_| LoginInvalid)?;

    match bcrypt::verify(password, &found.password) {
        Ok(true) => {
            let tokens = authorize(encoder.as_ref(), found.id).map_err(|_| LoginInvalid)?;
            let response = LoginResponse {
                user: found.into(),
                refresh_token: tokens.refresh_token,
                token: tokens.token,
            };

            Ok(HttpResponse::Ok().json(response))
        }
        _ => Err(actix_web::error::Error::from(LoginInvalid)),
    }
}

struct InsertReturn {
    id: uuid::Uuid,
}

#[post("/register/")]
async fn register(
    app: AppData,
    encoder: Data<AuthEncoder>,
    req: Json<CreateReq>,
) -> impl Responder {
    if let Err(validate) = req.validate() {
        return HttpResponse::BadRequest().json(validate);
    };

    let CreateReq {
        username,
        password,
        name,
        email,
    } = req.0;
    let password = match bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
        Ok(p) => p,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let id = query_as!(
        InsertReturn,
        "INSERT INTO users(username, password, name, email) VALUES($1, $2, $3, $4) RETURNING id",
        username,
        password,
        name,
        email
    )
    .fetch_one(&app.pool)
    .await;

    let id = match id {
        Ok(id) => id.id,
        Err(sqlx::Error::Database(_)) => return HttpResponse::Conflict().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let user = user::Response { name, username, id };
    let tokens = authorize(encoder.as_ref(), id).unwrap();
    HttpResponse::Created().json(LoginResponse {
        user,
        token: tokens.token,
        refresh_token: tokens.refresh_token,
    })
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").service(login).service(register));
}
