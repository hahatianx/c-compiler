mod utils;
mod common;
mod scanner;

use std::fs;

use crate::scanner::scanner::Scanner;
use crate::scanner::tokens::Token;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    dbg!(&args);


    let mut scanned_tokens: Vec<Token> = Vec::new();
    match fs::read_to_string(args[1].clone()) {
        Ok(text) => {
            let mut scanner = Scanner::new(text);
            match scanner.scan() {
                Ok(tokens) => {
                    println!("{:?}", tokens);
                    scanned_tokens = tokens;
                },
                Err(e) => panic!("{:?}", e),
            };
        }
        Err(e) => panic!("{:?}", e),
    };

}
