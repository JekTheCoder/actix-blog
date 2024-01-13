pub mod from_inline {
    pub trait FromInline: Sized {
        type Error: std::error::Error + Send + Sync + 'static;

        fn from_inline(inline: &str) -> Result<Self, Self::Error>;
    }
}

pub mod inline_vec {
    use crate::persistence::db::Database;

    use super::from_inline::FromInline;

    pub struct InlineVec<T>(Vec<T>);

    impl<T> InlineVec<T> {
        pub fn into_inner(self) -> Vec<T> {
            self.0
        }
    }

    impl<'r, T: FromInline> sqlx::Decode<'r, Database> for InlineVec<T> {
        fn decode(
            value: <Database as sqlx::database::HasValueRef<'r>>::ValueRef,
        ) -> Result<Self, sqlx::error::BoxDynError> {
            let row = value.as_str()?;
            let rows: Vec<_> = row.split(';').map(T::from_inline).collect::<Result<Vec<_>, _>>()?;

            Ok(Self(rows))
        }
    }
}
