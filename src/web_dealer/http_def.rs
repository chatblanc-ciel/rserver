use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, PartialOrd, PartialEq, std::cmp::Eq, Hash)]
pub enum RequestHeaderKind {
    MessageLength,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum HttpRequestMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Trace,
    Connect,
    Unknown,
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
impl Default for HttpRequestMethod {
    fn default() -> Self {
        Self::Unknown
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
    pub method: HttpRequestMethod,
    pub target: String,
    pub ver: String,
    pub header: HashMap<RequestHeaderKind, String>,
    pub body: String,
    pub remained_header: String,
}
impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            method: Default::default(),
            target: Default::default(),
            ver: Default::default(),
            header: HashMap::new(),
            body: Default::default(),
            remained_header: Default::default(),
        }
    }
}
impl TryInto<HttpRequest> for String {
    type Error = HttpError;

    fn try_into(self) -> Result<HttpRequest, Self::Error> {
        let mut header = HashMap::new();

        // bodyとheaderの分解
        let mut request = self
            .split("\r\n\r\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if request.len() != 2 {
            return Err(HttpError::RequestIsBroken);
        }
        let body = request.pop().ok_or(HttpError::RequestIsBroken)?;

        let request = request
            .pop()
            .ok_or(HttpError::RequestIsBroken)?
            .split("\r\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let request_first = request
            .get(0)
            .ok_or(HttpError::RequestIsBroken)?
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if request_first.len() != 3 {
            return Err(HttpError::RequestIsBroken);
        }

        for line in request.iter().skip(1) {
            let content = line.split(':').collect::<Vec<&str>>();

            match content.get(0).ok_or(HttpError::RequestIsBroken)?.trim() {
                "Content-Length" => {
                    header.insert(
                        RequestHeaderKind::MessageLength,
                        content
                            .get(1)
                            .ok_or(HttpError::RequestIsBroken)?
                            .trim()
                            .to_string(),
                    );
                }
                _ => {}
            }
        }

        Ok(HttpRequest {
            method: request_first[0].clone().try_into()?,
            target: request_first[1].clone(),
            ver: request_first[2].clone(),
            header,
            body,
            ..Default::default()
        })
    }
}

pub trait Response {}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum HttpResponseState {
    Complete, // `Ok` is Reserved word in Rust.
    NotFound,
}
impl HttpResponseState {
    pub fn code(&self) -> String {
        match self {
            Self::Complete => String::from("200"),
            Self::NotFound => String::from("404"),
        }
    }
}
impl Display for HttpResponseState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Complete => write!(f, "OK"),
            Self::NotFound => write!(f, "Not Found"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub state: HttpResponseState,
    pub ver: String,
    pub body: String,
}
impl Response for HttpResponse {}
impl From<HttpResponse> for String {
    fn from(res: HttpResponse) -> Self {
        format!(
            "{} {} {}\r\n\r\n{}",
            res.ver,
            res.state.code(),
            res.state,
            res.body
        )
    }
}

#[derive(Debug, Clone)]
pub enum HttpError {
    UndifineMethod,
    RequestIsBroken,
    FailGetControl,
    InternalErrorDiesel,
    InternalErrorTera,
}
impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndifineMethod => write!(f, "Http request method is undifined."),
            Self::RequestIsBroken => write!(f, "Http request is broken so can't deserialize."),
            Self::FailGetControl => write!(f, "Http get request is failed."),
            Self::InternalErrorDiesel => write!(f, "Internal database error occured."),
            Self::InternalErrorTera => write!(f, "Internal template engine error occured."),
        }
    }
}
impl Error for HttpError {}
