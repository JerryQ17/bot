use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new<T: ToString>(msg: &T) -> Error {
        Error { msg: msg.to_string() }
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BotError: {}", self.msg)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error { msg: value }
    }
}


impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::new(&err.to_string())
    }
}


impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::new(&err.to_string())
    }
}