#![allow(dead_code)]

extern crate downcast_rs;

mod utils;
mod common;
mod scanner;
mod parser;
mod source_code;
mod codegen;
mod interpreter;

use std::fs;
use crate::codegen::module::output::FileOutput;
use crate::codegen::x86::x86_generator::X86Generator;
use crate::interpreter::ast_interpreter::ASTInterpreter;
use crate::source_code::SourceCode;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    dbg!(&args);


    match fs::read_to_string(&args[1]) {
        Ok(text) => {
            let source_code = SourceCode::new(text);
            let mut scanner = source_code.into_scanner();
            let mut parser = source_code.into_parser(&mut scanner);


            let mut file = FileOutput::new("./output/output.txt").unwrap();
            let code_generator = Box::new(X86Generator::new(&mut file));
            let mut interpreter = ASTInterpreter::new(code_generator);

            match parser.parse() {
                Ok(ast) => {
                    // println!("{:#?}", ast)
                    interpreter.interpret(&*ast).expect("Failed to interpret");

                },
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
