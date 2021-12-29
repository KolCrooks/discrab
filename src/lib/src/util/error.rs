use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub code: ErrorTypes,
}

impl Error {
    pub fn new(message: String, code: ErrorTypes) -> Error {
        Error { message, code }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error ({:?}) - {}", self.code, self.message)
    }
}

#[derive(Debug)]
pub enum ErrorTypes {
    PARSE,
    REQUEST,
}
