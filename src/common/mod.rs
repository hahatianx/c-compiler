use crate::common::errors::error::CompilerError;

pub mod errors;

pub type Result<T> = std::result::Result<T, dyn CompilerError>;