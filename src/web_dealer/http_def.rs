use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum HttpRequestMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Trace,
    Connect,
}
impl Display for HttpRequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            _ => unimplemented!(),
        }
    }
}
impl From<HttpRequestMethod> for String {
    fn from(value: HttpRequestMethod) -> Self {
        format!("{}", value)
    }
}
impl TryInto<HttpRequestMethod> for String {
    type Error = HttpError;

    fn try_into(self) -> Result<HttpRequestMethod, Self::Error> {
        match self.as_str() {
            "GET" => Ok(HttpRequestMethod::Get),
            "POST" => Ok(HttpRequestMethod::Post),
            "PUT" => Ok(HttpRequestMethod::Put),
            "DELETE" => Ok(HttpRequestMethod::Delete),
            _ => Err(HttpError::UndifineMethod),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    method: HttpRequestMethod,
    target: String,
    ver: String,
}

#[derive(Debug, Clone)]
pub enum HttpError {
    UndifineMethod,
}
impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndifineMethod => write!(f, "Http request method is undifined."),
        }
    }
}
impl Error for HttpError {}
