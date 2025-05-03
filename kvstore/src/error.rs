use std::fmt;

#[derive(Debug)]
pub enum KvError {
    NotFound,
    Io(std::io::Error),
    Other(String),
}

impl std::error::Error for KvError {}

impl fmt::Display for KvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KvError::NotFound => write!(f, "Key not found"),
            KvError::Io(err) => write!(f, "IO error: {}", err),
            KvError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

pub type KvResult<T> = Result<T, KvError>;