mod utils;
mod common;
mod scanner;
mod parser;
mod source_code;

use std::fs;

use crate::scanner::scanner::Scanner;
use crate::scanner::tokens::{Token, TokenType};
use crate::source_code::SourceCode;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    dbg!(&args);


    match fs::read_to_string(args[1].clone()) {
        Ok(text) => {
            let source_code = SourceCode::new(text);
            let mut scanner = source_code.into_scanner();
            let mut parser = source_code.into_parser(&mut scanner);

            match parser.parse() {
                Ok(ast) => println!("{:#?}", ast),
                Err(e) => panic!("{:?}", e),
            }


            // loop {
            //     match scanner.scan() {
            //         Ok(token) => {
            //             println!("{:?}", token);
            //             if token.get_token_type() == TokenType::Eof {
            //                 break;
            //             }
            //         },
            //         Err(e) => panic!("{:?}", e),
            //     }
            // }
        }
        Err(e) => panic!("{:?}", e),
    };

}
