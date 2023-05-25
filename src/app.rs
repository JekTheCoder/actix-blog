use sqlx::{PgPool, Postgres};

pub type Pool = PgPool;
pub type QueryResult = <Postgres>::QueryResult;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

pub type AppData = actix_web::web::Data<AppState>;
