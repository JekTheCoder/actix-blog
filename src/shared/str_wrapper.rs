macro_rules! super_str {
    ($name:ident) => {
        impl $name {
            pub const unsafe fn unchecked_from_str(slice: &str) -> &Self {
                &*(slice as *const _ as *const Self)
            }

            pub unsafe fn from_boxed_unchecked(v: Box<str>) -> Box<Self> {
                unsafe { Box::from_raw(Box::into_raw(v) as *mut Self) }
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

pub(crate) use super_str;
