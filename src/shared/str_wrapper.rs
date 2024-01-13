macro_rules! super_str {
    ($name:ident) => {
        #[allow(dead_code)]
        impl $name {
            pub const unsafe fn unchecked_from_str(slice: &str) -> &Self {
                &*(slice as *const _ as *const Self)
            }

            pub fn new(
                slice: &str,
            ) -> Result<&Self, <Self as crate::shared::str_wrapper::CheckStr>::Error> {
                Self::check_str(slice)?;
                unsafe { Ok(Self::unchecked_from_str(slice)) }
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                &self.0 == other
            }
        }
    };
}

macro_rules! buf_ops {
    ($buf_name: ident, $slice_name: ident) => {
        impl $buf_name {
            pub fn from_boxed_unchecked(boxed: Box<str>) -> Self {
                let raw = Box::into_raw(boxed);
                let converted = unsafe { Box::from_raw(raw as *mut _) };
                Self(converted)
            }

            #[allow(dead_code)]
            pub fn as_str(&self) -> &str {
                self.as_ref().as_ref()
            }
        }

        impl std::ops::Deref for $buf_name {
            type Target = $slice_name;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<$slice_name> for $buf_name {
            fn as_ref(&self) -> &$slice_name {
                &self.0
            }
        }

        impl TryFrom<Box<str>> for $buf_name {
            type Error = <$slice_name as crate::shared::str_wrapper::CheckStr>::Error;

            fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
                <$slice_name as crate::shared::str_wrapper::CheckStr>::check_str(value.as_ref())?;
                Ok(Self::from_boxed_unchecked(value))
            }
        }

        impl TryFrom<String> for $buf_name {
            type Error = <$slice_name as crate::shared::str_wrapper::CheckStr>::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.into_boxed_str().try_into()
            }
        }
    };
}

pub(crate) use buf_ops;
pub(crate) use super_str;

pub trait CheckStr {
    type Error;

    fn check_str(slice: &str) -> Result<(), Self::Error>;
}
