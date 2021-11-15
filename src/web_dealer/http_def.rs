use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum HttpRequest {
    Get(String),
    Post,
    Put,
    Delete,
    Head,
    Options,
    Trace,
    Connect,
}
impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get(dir) => write!(f, "GET {} HTTP/1.1\r\n", dir),
            _ => unimplemented!(),
        }
    }
}
