pub mod preview {
    use std::fmt::Display;

    use crate::shared::str_wrapper::super_str;

    #[derive(Clone, Debug)]
    pub enum Error {}

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    #[repr(transparent)]
    pub struct Preview(str);

    super_str!(Preview);

    impl Preview {
        pub const fn preview(slice: &str) -> Result<&Self, Error> {
            Ok(unsafe { Self::unchecked_from_str(slice) })
        }

        pub fn from_boxed(boxed: Box<str>) -> Result<Box<Self>, Error> {
            Ok(unsafe { Self::from_boxed_unchecked(boxed) })
        }
    }

    #[derive(Debug)]
    pub struct PreviewBuf(Box<Preview>);

    impl std::ops::Deref for PreviewBuf {
        type Target = Preview;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Clone for PreviewBuf {
        fn clone(&self) -> Self {
            let buf: Box<str> = self.0.as_ref().as_ref().into();
            let other = unsafe { Preview::from_boxed_unchecked(buf) };
            Self(other)
        }
    }

    impl<'de> serde::Deserialize<'de> for PreviewBuf {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let data = String::deserialize(deserializer)?;
            match Preview::from_boxed(data.into_boxed_str()) {
                Ok(preview) => Ok(Self(preview)),
                Err(e) => Err(serde::de::Error::custom(e)),
            }
        }
    }
}
