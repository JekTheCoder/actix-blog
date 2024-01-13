use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HeadlessTag {
    pub id: uuid::Uuid,
    pub name: String,
    pub color: String,
}

mod tag_from_inline {
    use uuid::Uuid;

    use crate::persistence::db::decode::from_inline::FromInline;

    use super::HeadlessTag;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("id")]
        Id,
        #[error("name")]
        Name,
        #[error("color")]
        Color,
    }

    impl FromInline for HeadlessTag {
        type Error = Error;

        fn from_inline(inline: &str) -> Result<Self, Self::Error> {
            let mut parts = inline.split(',');
            let Ok(id) = Uuid::parse_str(parts.next().ok_or(Error::Id)?) else {
                return Err(Error::Id);
            };

            let Ok(name) = parts.next().ok_or(Error::Name) else {
                return Err(Error::Name);
            };

            let Ok(color) = parts.next().ok_or(Error::Color) else {
                return Err(Error::Color);
            };

            Ok(Self {
                id,
                name: name.to_owned(),
                color: color.to_owned(),
            })
        }
    }
}
