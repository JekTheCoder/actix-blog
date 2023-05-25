use sqlx::{postgres::PgPoolOptions, Database, PgPool, Postgres};

pub type Pool = PgPool;
pub type QueryResult = <Postgres as Database>::QueryResult;
pub type PoolOptions = PgPoolOptions;
