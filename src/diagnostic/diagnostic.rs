use std::fmt;
pub enum ErrorKind {
    Lexical,
    Syntax,
    Runtime,
} 

pub struct ErrorHandler<'a> {
    err_type: ErrorKind,
    error: &'a str,
    line: usize,
}

impl<'a> ErrorHandler<'a> {
    pub fn new(err_type: ErrorKind, error: &'a str, line: usize) -> Self {
        Self {err_type, error, line}
    }
}

impl<'a> fmt::Display for ErrorHandler<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.err_type {
            ErrorKind::Lexical => "Lexical Error",
            ErrorKind::Syntax  => "Syntax Error",
            ErrorKind::Runtime => "Runtime Error",
        };
        write!(f, "[{}] on line {}: {}", prefix, self.line, self.error)
    }
}