use sqlx::{PgPool, Postgres, Database};

pub type Pool = PgPool;
pub type QueryResult = <Postgres as Database>::QueryResult;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

pub type AppData = actix_web::web::Data<AppState>;
