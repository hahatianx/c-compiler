use std::fs::File;
use std::io::Write;
use crate::common::errors::error::CompilerErrorKind;
use crate::common::Result;

pub struct FileOutput {

    file: File,

}

#[macro_export]
macro_rules! file_writeln {
    ( $self:ident, $output:ident, $f:ident, $($x:expr),* $(,)? ) => {
        $(
            $self.$output.$f($x)?;
        )*
    };
}

impl FileOutput {

    pub fn new(path: &str) -> Result<FileOutput> {
        match File::options().write(true).create(true).truncate(true).open(path) {
            Ok(file) => {
                Ok(Self {
                    file,
                })
            },
            Err(e) => {
                Err(CompilerErrorKind::FileError(e.to_string()).into())
            }

        }
    }

    pub fn writeln(&mut self, line: &str) -> Result<()> {
        let line = format!("{}\n", line);
        match self.file.write_all(line.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(CompilerErrorKind::FileError(e.to_string()).into()),
        }
    }

}