use std::fmt::{self};

pub enum ErrorCode {
    SyntaxError,
    CompilerError
}
impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::SyntaxError => write!(f, "SyntaxError"),
            ErrorCode::CompilerError => write!(f, "CompilerError")
        }
    }
}