use crate::scanner::scanner::Scanner;
use crate::parser;
use crate::parser::parser::Parser;

pub struct SourceCode {

    code: String,

}


impl SourceCode {
    pub fn new(code: String) -> SourceCode {
        Self {
            code
        }
    }

    pub fn get_source_code(&self) -> &String {
        &self.code
    }

    pub fn into_scanner(&self) -> Scanner {
        Scanner::new(self)
    }

    pub fn into_parser<'a>(&'a self, scanner: &'a mut Scanner<'a>) -> Parser<'a> {
        Parser::new(scanner)
    }

}