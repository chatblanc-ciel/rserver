use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum DealerError {
    CannotListen,
}
impl Display for DealerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CannotListen => write!(f, "Can not listen for address"),
        }
    }
}
impl Error for DealerError {}
