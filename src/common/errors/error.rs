use std::fmt::{Debug, Display, Formatter};

pub enum CompilerErrorKind {
    ScannerError(usize, usize, String),
}

impl Display for CompilerErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerErrorKind::ScannerError(line, column, message) => {
                write!(f, "Scanner Error (line: {}, column: {}, message: {})", line, column, message)
            }
        }
    }
}

impl Debug for CompilerErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerErrorKind::ScannerError(line, column, message) => {
                write!(f, "Scanner Error (line: {}, column: {}, message: {})", line, column, message)
            }
        }
    }
}