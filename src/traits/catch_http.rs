use super::into_http_err::IntoHttpErr;

pub trait CatchHttp<T> {
    type Err;
    fn catch_http(self) -> Result<T, Self::Err>;
}

impl<T, E> CatchHttp<T> for Result<T, E>
where
    E: IntoHttpErr,
{
    type Err = E::Err;
    fn catch_http(self) -> Result<T, <Result<T, E> as CatchHttp<T>>::Err> {
        self.map_err(|e| e.http_err())
    }
}
