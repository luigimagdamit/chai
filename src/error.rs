use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
pub enum ErrorCode {
    SyntaxError,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            Self::SyntaxError => write!(f, "SyntaxError"),
        }
    }
}
