pub mod preview {
    use std::fmt::Display;

    #[derive(Clone, Debug)]
    pub enum Error {
        Empty,
        MaxLen,
    }

    #[derive(Debug, Clone)]
    pub struct PreviewBuf(Box<str>);

    impl Display for PreviewBuf {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl PreviewBuf {
        pub fn validate(str: &str) -> Result<(), Error> {
            if str.is_empty() {
                return Err(Error::Empty);
            }

            if str.len() > 400 {
                return Err(Error::MaxLen);
            }

            Ok(())
        }

        pub fn from_boxed_unchecked(boxed: Box<str>) -> Self {
            Self(boxed)
        }
    }

    impl AsRef<str> for PreviewBuf {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl TryFrom<Box<str>> for PreviewBuf {
        type Error = Error;
        fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
            Self::validate(value.as_ref())?;

            Ok(Self(value))
        }
    }

    impl TryFrom<String> for PreviewBuf {
        type Error = Error;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.into_boxed_str().try_into()
        }
    }
}

pub mod content {
    use std::fmt::Display;

    #[derive(Clone, Debug)]
    pub enum Error {
        Empty,
    }

    #[derive(Debug, Clone)]
    pub struct ContentBuf(Box<str>);

    impl Display for ContentBuf {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl ContentBuf {
        pub fn validate(str: &str) -> Result<(), Error> {
            if str.is_empty() {
                return Err(Error::Empty);
            }

            Ok(())
        }

        pub fn from_boxed_unchecked(boxed: Box<str>) -> Self {
            Self(boxed)
        }
    }

    impl AsRef<str> for ContentBuf {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl TryFrom<Box<str>> for ContentBuf {
        type Error = Error;
        fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
            Self::validate(value.as_ref())?;

            Ok(Self(value))
        }
    }

    impl TryFrom<String> for ContentBuf {
        type Error = Error;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.into_boxed_str().try_into()
        }
    }
}
