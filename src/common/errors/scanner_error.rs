use std::fmt::{Display, Formatter};
use crate::common::errors::error::CompilerError;

#[derive(Debug, Clone)]
pub struct ScannerError {

    pub line: usize,
    pub message: String,

}

impl Display for ScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

impl CompilerError for ScannerError {}

impl ScannerError {
    pub fn new(line: usize, message: String) -> Self {
        Self { line, message }
    }
}