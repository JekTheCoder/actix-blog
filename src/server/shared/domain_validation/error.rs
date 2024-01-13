use std::{borrow::Cow, collections::HashMap};

#[derive(serde::Serialize, Debug)]
pub struct FieldError(Cow<'static, str>);

#[derive(serde::Serialize, Debug, Default)]
pub struct FieldErrors(Vec<FieldError>);

#[derive(serde::Serialize, Debug, Default)]
pub struct StructErrors(HashMap<Cow<'static, str>, Error>);

#[derive(serde::Serialize, Debug)]
#[serde(untagged)]
pub enum Error {
    Field(FieldErrors),
    Struct(StructErrors),
}

impl FieldErrors {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl StructErrors {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add(&mut self, field: impl Into<Cow<'static, str>>, error: impl Into<Error>) {
        self.0.insert(field.into(), error.into());
    }
}

impl FieldError {
    pub fn minlen(got: usize, expected: usize) -> Self {
        Self(format!("min length is {}, got: {}", expected, got).into())
    }

    pub fn maxlen(got: usize, expected: usize) -> Self {
        Self(format!("max length is {}, got: {}", expected, got).into())
    }

    pub fn len(got: usize, expected: usize) -> Self {
        Self(format!("length is {}, expected: {}", got, expected).into())
    }

    pub fn email() -> Self {
        Self("invalid email".into())
    }

    pub fn custom(msg: impl Into<Cow<'static, str>>) -> Self {
        Self(msg.into())
    )
}

impl FieldErrors {
    pub fn add(&mut self, error: impl Into<FieldError>) {
        self.0.push(error.into())
    }
}

impl From<StructErrors> for Error {
    fn from(value: StructErrors) -> Self {
        Self::Struct(value)
    }
}

impl From<FieldErrors> for Error {
    fn from(value: FieldErrors) -> Self {
        Self::Field(value)
    }
}

mod display {
    use std::fmt::Display;

    use actix_web::{http::header::ContentType, HttpResponse};

    impl Display for super::Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match serde_json::to_string_pretty(self) {
                Ok(json) => write!(f, "{}", json),
                Err(_) => write!(f, "internal error"),
            }
        }
    }

    impl actix_web::ResponseError for super::Error {
        fn status_code(&self) -> actix_web::http::StatusCode {
            actix_web::http::StatusCode::BAD_REQUEST
        }

        fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
            HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .json(self)
        }
    }
}
