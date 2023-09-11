use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error { msg: msg.to_string() }
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BotError: {}", self.msg)
    }
}


impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::new(&err.to_string())
    }
}


impl TryInto<std::io::Error> for Error {
    type Error = std::io::Error;

    fn try_into(self) -> Result<std::io::Error, Self::Error> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, self.msg))
    }
}
