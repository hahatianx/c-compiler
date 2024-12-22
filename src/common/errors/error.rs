use std::fmt::{write, Debug, Display, Formatter};

pub enum CompilerErrorKind {
    InternalError(String),
    ScannerError(usize, usize, String),
    CompilerError(usize, usize, String),
}

fn fmt(error: &CompilerErrorKind, f: &mut Formatter<'_>) -> std::fmt::Result {
    match error {
        CompilerErrorKind::ScannerError(line, column, message) => {
            write!(f, "Scanner Error (line: {}, column: {}, message: {})", line, column, message)
        },
        CompilerErrorKind::CompilerError(line, column, message) => {
            write!(f, "Compiler Error: (line: {}, column: {}, message: {})", line, column, message)
        },
        CompilerErrorKind::InternalError(message) => {
            write!(f, "Internal Error: {}", message)
        }
    }
}

impl Display for CompilerErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt(self, f)
    }
}

impl Debug for CompilerErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt(self, f)
    }
}