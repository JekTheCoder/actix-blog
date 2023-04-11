#[derive(Debug, thiserror::Error)]
pub enum InsertError {
    #[error("Unknown error")]
    Unknown,
    #[error("No insert error")]
    NoInsert,
}

impl From<sqlx::Error> for InsertError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => InsertError::NoInsert,
            _ => InsertError::Unknown,
        }
    }
}

