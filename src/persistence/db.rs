pub mod entities;
pub mod decode;

use actix_web::web::{Data, ServiceConfig};
use sqlx::{self, migrate, postgres::PgPoolOptions, PgPool, Postgres};

use crate::server::AppConfig;

pub type Database = Postgres;
pub type Pool = PgPool;
pub type QueryResult = <Database as sqlx::Database>::QueryResult;
pub type PoolOptions = PgPoolOptions;
pub type Driver = Postgres;
pub type DateTime = chrono::NaiveDateTime;

pub trait Executor<'a>: sqlx::Executor<'a, Database = Database> {}

impl<'a, E> Executor<'a> for E where E: sqlx::Executor<'a, Database = Database> {}

pub use slice::Slice;

#[derive(Clone)]
pub struct DbConfig(Pool);

impl DbConfig {
    pub async fn new() -> Self {
        let database = dotenvy::var("DATABASE_URL").expect("DATABASE could not load");

        let pool = PoolOptions::new()
            .max_connections(10)
            .connect(&database)
            .await
            .expect("Pg pool not conected");

        migrate!("./migrations")
            .run(&pool)
            .await
            .expect("could not run migrations");

        Self(pool)
    }
}

impl AppConfig for DbConfig {
    fn configure(self, config: &mut ServiceConfig) {
        config.app_data(Data::new(self.0));
    }
}

mod slice {
    use crate::server::shared::query::QuerySlice;

    pub struct Slice {
        pub limit: i64,
        pub offset: i64,
    }

    impl From<QuerySlice> for Slice {
        fn from(slice: QuerySlice) -> Self {
            Self {
                limit: slice.limit as i64,
                offset: slice.offset as i64,
            }
        }
    }
}
