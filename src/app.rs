use sqlx::PgPool;

pub type Pool = PgPool;
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

pub type AppData = actix_web::web::Data<AppState>;
