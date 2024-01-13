use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HeadlessSubCategory {
    pub id: uuid::Uuid,
    pub name: String,
}

mod from_inline {
    use uuid::Uuid;

    use crate::persistence::db::decode::from_inline::FromInline;

    use super::HeadlessSubCategory;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("id")]
        Id,
        #[error("name")]
        Name,
    }

    impl FromInline for HeadlessSubCategory {
        type Error = Error;

        fn from_inline(inline: &str) -> Result<Self, Self::Error> {
            let mut parts = inline.split(',');
            let Ok(id) = Uuid::parse_str(parts.next().ok_or(Error::Id)?) else {
                return Err(Error::Id);
            };

            let Ok(name) = parts.next().ok_or(Error::Name) else {
                return Err(Error::Name);
            };

            Ok(Self {
                id,
                name: name.to_owned(),
            })
        }
    }
}
