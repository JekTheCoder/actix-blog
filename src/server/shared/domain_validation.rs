pub mod error;
pub use error::{Error, FieldError, FieldErrors};

mod domain_impls {
    use markdown_parse::{content::ContentBuf, preview::PreviewBuf};

    use super::{DomainValid, FieldError, FieldErrors};

    impl DomainValid for PreviewBuf {
        type Unchecked = String;

        fn from_unchecked(unchecked: Self::Unchecked) -> Result<Self, super::error::Error> {
            let unchecked_len = unchecked.len();

            unchecked.try_into().map_err(|err| {
                let field_err = match err {
                    markdown_parse::preview::Error::Empty => FieldError::minlen(unchecked_len, 1),
                    markdown_parse::preview::Error::MaxLen => {
                        FieldError::maxlen(unchecked_len, 400)
                    }
                };

                let mut errs = FieldErrors::default();
                errs.add(field_err);

                super::error::Error::Field(errs)
            })
        }
    }

    impl DomainValid for ContentBuf {
        type Unchecked = String;

        fn from_unchecked(unchecked: Self::Unchecked) -> Result<Self, super::error::Error> {
            let unchecked_len = unchecked.len();

            unchecked.try_into().map_err(|err| {
                let field_err = match err {
                    markdown_parse::content::Error::Empty => FieldError::minlen(unchecked_len, 1),
                };

                let mut errs = FieldErrors::default();
                errs.add(field_err);

                super::error::Error::Field(errs)
            })
        }
    }
}

pub trait DomainValid: Sized {
    type Unchecked;

    fn from_unchecked(unchecked: Self::Unchecked) -> Result<Self, error::Error>;
}

macro_rules! dumb_impl_domain_valid {
    ($($t: ty), *) => {
        $(
            impl DomainValid for $t
            {
                type Unchecked = $t;

                fn from_unchecked(unchecked: Self::Unchecked) -> Result<Self, error::Error> {
                    Ok(unchecked)
                }
            }
        )*
    };
}

dumb_impl_domain_valid!(
    String,
    Box<str>,
    uuid::Uuid,
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
    bool,
    char
);

macro_rules! destructure {
    ($path: path; $name: ident; $($field: ident : $ty: ty,)* ) => {
        let $path {
            $($field),*
        } = $name;
    };
    ($name: ident; $($field: ident : $ty: ty,)* ) => {
        $name {
            $($field),*
        }
    }
}

macro_rules! gen_unchecked {
    (@ $name: ident; { } [ $($parsed: tt)* ]) => {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $($parsed)*
        }
    };
    (@ $name: ident; { $field: ident : Option<$ty: ty> , $($rest: tt)* } [ $($parsed: tt)* ]) => {
        crate::server::shared::domain_validation::gen_unchecked!(@ $name; { $($rest)* } [ $($parsed)* pub $field: Option<<$ty as crate::server::shared::domain_validation::DomainValid>::Unchecked>, ]);
    };
    (@ $name: ident; { $field: ident : Vec<$ty: ty> , $($rest: tt)* } [ $($parsed: tt)* ]) => {
        crate::server::shared::domain_validation::gen_unchecked!(@ $name; { $($rest)* } [ $($parsed)* pub $field: Vec<<$ty as crate::server::shared::domain_validation::DomainValid>::Unchecked>, ]);
    };
    (@ $name: ident; { $field: ident : $ty: ty , $($rest: tt)* } [ $($parsed: tt)* ]) => {
        crate::server::shared::domain_validation::gen_unchecked!(@ $name; { $($rest)* } [ $($parsed)* pub $field: <$ty as crate::server::shared::domain_validation::DomainValid>::Unchecked, ]);
    };
    (struct $name: ident { $($body: tt)* } ) => {
        crate::server::shared::domain_validation::gen_unchecked!(@ $name; { $($body)* } [ ]);
    };
}

macro_rules! impls {
    (@ $name: ident; $errors: ident; { } [ $($parsed: tt)* ]) => {
        $($parsed)*
    };
    (@ $name: ident; $errors: ident; { $field: ident : Option<$ty: ty> , $($rest: tt)* } [ $($parsed: tt)* ]) => {
        crate::server::shared::domain_validation::impls!(@ $name; $errors; { $($rest)* } [ $($parsed)*
            let $field = match $field {
                Some(val) => {
                    let checked = <$ty as crate::server::shared::domain_validation::DomainValid>::from_unchecked(val);
                    match checked {
                        Ok(val) => Some(val),
                        Err(e) => {
                            $errors.add(stringify!($field), e);
                            return Err($errors.into());
                        }
                    }
                },
                None => None,
            };
        ]);
    };
    (@ $name: ident; $errors: ident; { $field: ident : Vec<$ty: ty> , $($rest: tt)* } [ $($parsed: tt)* ]) => {
        crate::server::shared::domain_validation::impls!(@ $name; $errors; { $($rest)* } [ $($parsed)*
            let $field = {
                let mut vals = Vec::with_capacity($field.len());
                for (i, val) in $field.into_iter().enumerate() {
                    match <$ty as crate::server::shared::domain_validation::DomainValid>::from_unchecked(val) {
                        Ok(val) => vals.push(val),
                        Err(e) => $errors.add(i.to_string(), e)
                    };
                }

                if !$errors.is_empty() {
                    return Err($errors.into());
                }

                vals
            };
        ]);
    };
    (@ $name: ident; $errors: ident; { $field: ident : $ty: ty , $($rest: tt)* } [ $($parsed: tt)* ]) => {
        crate::server::shared::domain_validation::impls!(@ $name; $errors; { $($rest)* } [ $($parsed)*
            let $field = {
                let validation = <$ty as crate::server::shared::domain_validation::DomainValid>::from_unchecked($field);

                match validation {
                    Ok(val) => val,
                    Err(e) => {
                        $errors.add(stringify!($field), e);
                        return Err($errors.into());
                    }
                }
            };
        ]);
    };
    ($name: ident; $alter_name: ident; { $($body: tt)* } ) => {
        impl crate::server::shared::domain_validation::DomainValid for $name {
            type Unchecked = $alter_name;

            fn from_unchecked(unchecked: Self::Unchecked) -> Result<Self, crate::server::shared::domain_validation::error::Error> {
                crate::server::shared::domain_validation::destructure!(Self::Unchecked; unchecked; $($body)*);
                let mut errors = <crate::server::shared::domain_validation::error::StructErrors as Default>::default();

                crate::server::shared::domain_validation::impls!(@ $name; errors; { $($body)* } [ ]);

                Ok(crate::server::shared::domain_validation::destructure!(Self; $($body)*))
            }
        }
    };
}

macro_rules! pub_fields {
        ($name: ident; $($field: ident : $ty: ty,)*) => {
            pub struct $name {
                $(pub $field: $ty,)*
            }
        };
    }

macro_rules! domain_valid {
    (pub struct $name: ident { $($body: tt)* }; $alter_name: ident) => {
        crate::server::shared::domain_validation::pub_fields!($name; $($body)*);

        crate::server::shared::domain_validation::gen_unchecked!(struct $alter_name { $($body)* });

        crate::server::shared::domain_validation::impls!($name; $alter_name; { $($body)* });
    };
}

pub(crate) use destructure;
pub(crate) use domain_valid;
pub(crate) use gen_unchecked;
pub(crate) use impls;
pub(crate) use pub_fields;
