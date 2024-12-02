use crate::common::errors::error::CompilerErrorKind;

pub mod errors;

pub type Result<T> = core::result::Result<T, CompilerErrorKind>;