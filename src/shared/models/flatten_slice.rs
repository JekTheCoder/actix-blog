use serde::Deserialize;

#[derive(Debug)]
pub struct FlattenSlice {
    /// Defaults to 20
    pub limit: u32,
    /// Defaults to 0
    pub offset: u32,
}

fn parse_u32<'de, D: serde::Deserializer<'de>>(
    s: Option<&'_ str>,
    def: u32,
) -> Result<u32, D::Error> {
    let Some(s) = s else {
        return Ok(def);
    };

    s.parse().map_err(serde::de::Error::custom)
}

impl<'de> Deserialize<'de> for FlattenSlice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawSlice<'a> {
            #[serde(borrow)]
            pub limit: Option<&'a str>,
            #[serde(borrow)]
            pub offset: Option<&'a str>,
        }

        let RawSlice { limit, offset } = RawSlice::deserialize(deserializer)?;
        let limit = parse_u32::<D>(limit, 20)?;
        let offset = parse_u32::<D>(offset, 0)?;

        Ok(Self { limit, offset })
    }
}

#[cfg(test)]
mod tests {
    use actix_web::web::Query;

    use super::*;

    #[test]
    fn can_deserialize_as_standalone() {
        let query = "limit=10&offset=20";
        let Query(FlattenSlice { limit, offset }) = Query::from_query(&query).unwrap();

        assert_eq!(limit, 10);
        assert_eq!(offset, 20);
    }

    #[test]
    fn can_deserialize_as_flattened() {
        #[derive(Deserialize)]
        struct Extended {
            my_arg: u32,
            #[serde(flatten)]
            slice: FlattenSlice,
        }

        let query = "my_arg=2&limit=10&offset=20";
        let Query(Extended { my_arg, slice }) = Query::from_query(&query).unwrap();

        assert_eq!(my_arg, 2);
        assert_eq!(slice.limit, 10);
        assert_eq!(slice.offset, 20);
    }

    #[test]
    fn can_deserialize_as_partial_flattened() {
        #[derive(Deserialize)]
        struct Extended {
            my_arg: u32,
            #[serde(flatten)]
            slice: FlattenSlice,
        }

        let query = "my_arg=2&offset=12";
        let Query(Extended { my_arg, slice }) = Query::from_query(&query).unwrap();

        assert_eq!(my_arg, 2);
        assert_eq!(slice.limit, 20);
        assert_eq!(slice.offset, 12);
    }
}
