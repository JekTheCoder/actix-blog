use super::{into_http::IntoHttp, into_http_err::IntoHttpErr};

pub trait IntoResponse<T> {
    fn into_response(self) -> Result<T, actix_web::Error>;
}

impl<T, E> IntoResponse<T::Http> for Result<T, E>
where
    T: IntoHttp,
    E: IntoHttpErr,
{
    fn into_response(self) -> Result<T::Http, actix_web::Error> {
        match self {
            Ok(t) => Ok(t.into_http()),
            Err(e) => Err(e.http_err().into()),
        }
    }
}
