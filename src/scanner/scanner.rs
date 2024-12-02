use crate::common::errors::error::CompilerErrorKind;
use crate::common::Result;
use crate::scanner::number_parser::{NumberParser};
use crate::scanner::tokens::{Token, TokenType};
use crate::utils::trie::KeywordTrie;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner {
    source_code: String,
    keyword_trie: KeywordTrie,
    number_parser: NumberParser,
}

struct ScannerPeekable<'a> {
    chars: Peekable<Chars<'a>>,

    line_number: usize,
    col_number: usize,
}

impl<'a> ScannerPeekable<'a> {
    fn new(chars: Peekable<Chars<'a>>) -> Self {
        Self {
            chars,
            line_number: 1,
            col_number: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.col_number += 1;
        let result = self.chars.next();
        if result == Some('\n') {
            self.line_number += 1;
            self.col_number = 0;
        }

        result
    }

    fn c_match(&mut self, target: char) -> bool {
        match self.peek() {
            None => false,
            Some(c) => {
                if *c == target {
                    self.next();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
}

fn parse_constant_chars(
    chars: &mut ScannerPeekable,
    target: char,
) -> Result<String> {
    let mut result = String::new();
    let mut have_slash: bool = false;
    while let Some(next_char) = chars.peek() {
        if *next_char == '\n' {
            // TODO: raise errors, string / char constants does not allow new spaces
            return Err(CompilerErrorKind::ScannerError(
                chars.line_number,
                chars.col_number,
                String::from("Cannot have '\n' between quotes."),
            ));
        } else {
            if *next_char == '\\' {
                have_slash = !have_slash;
            } else if *next_char == target {
                if !have_slash {
                    chars.next();
                    break;
                }
                have_slash = false;
            }
        }
        result.push(chars.next().unwrap());
        // TODO: add this char into the constant scope
    }
    Ok(result)
}

impl Scanner {
    pub fn new(code: String) -> Self {
        Self {
            keyword_trie: KeywordTrie::new(),
            number_parser: NumberParser::new(),
            source_code: code,
        }
    }



    pub fn scan(&mut self) -> Result<Vec<Token>> {

        let mut tokens: Vec<Token> = Vec::new();

        let mut block_comment = 0;
        let mut line_comment = 0;

        let mut keyword_checker = self.keyword_trie.into_checker();
        let mut number_checker = self.number_parser.into_checker();

        let mut chars = ScannerPeekable::new(self.source_code.chars().peekable());
        loop {
            match chars.next() {
                Some(c) => {

                    if line_comment > 0 {
                        if c == '\n' {
                            line_comment = 0;
                        }
                        continue;
                    }
                    if block_comment > 0 {
                        if c == '*' && chars.c_match('/') {
                            block_comment -= 1;
                        }
                        continue;
                    }

                    if c.is_whitespace() {
                    } else if c.is_ascii_alphabetic() || c == '_' {
                        // allowed chars [a-zA-Z_]{1}[a-zA-Z_0-9]*
                        keyword_checker.update(c);
                        while let Some(next_chars) = chars.peek() {
                            if !keyword_checker.can_consume(next_chars) {
                                break;
                            }
                            keyword_checker.update(chars.next().unwrap());
                        }
                        if let Some(keyword) = keyword_checker.check() {
                            tokens.push(Token::single_token(keyword));
                        } else {
                            // TODO: identifier tokens
                            match keyword_checker.get_str() {
                                Ok(str) => tokens.push(Token::identifier(&str)),
                                Err(error) => return Err(error),
                            }
                        }
                    } else if c.is_digit(10) {
                        number_checker.update(c);
                        // allowed chars [0-9]{1}[0-9a-z.]*, delegate to number checker for validation checks
                        while let Some(next_chars) = chars.peek() {
                            if !number_checker.can_consume(next_chars) {
                                break;
                            }
                            number_checker.update(chars.next().unwrap());
                        }
                        if let Ok(value) = number_checker.check() {
                            // a valid token
                            tokens.push(Token::number_token(value));
                        } else {
                            // TODO: raise number parse errors
                        }
                    } else if c == '"' || c == '\'' {
                        // TODO: string constants
                        match parse_constant_chars(&mut chars, c) {
                            Ok(str) => {
                                match c {
                                    '\'' => tokens.push(Token::text_token(TokenType::Char, &str)),
                                    '"' => tokens.push(Token::text_token(TokenType::String, &str)),
                                    _ => {panic!()}
                                }
                            }
                            Err(err) => return Err(err)
                        }
                    } else {
                        match c {
                            '(' => tokens.push(Token::single_token(TokenType::LeftParen)),
                            ')' => tokens.push(Token::single_token(TokenType::RightParen)),
                            '{' => tokens.push(Token::single_token(TokenType::LeftBrace)),
                            '}' => tokens.push(Token::single_token(TokenType::RightBrace)),
                            ',' => tokens.push(Token::single_token(TokenType::Comma)),
                            ';' => tokens.push(Token::single_token(TokenType::Semicolon)),
                            '+' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::PlusEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::Plus));
                                }
                            }
                            '-' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::MinusEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::Minus));
                                }
                            }
                            '*' => {
                                if chars.c_match('/') {
                                    return Err(CompilerErrorKind::ScannerError(
                                        chars.line_number,
                                        chars.col_number,
                                        String::from("unrecognized end of comment '*/'")
                                    ))
                                } else if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::StarEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::Star));
                                }
                            }
                            '/' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::SlashEqual));
                                } else if chars.c_match('/') {
                                    line_comment += 1;
                                } else if chars.c_match('*'){
                                    block_comment += 1;
                                } else {
                                    tokens.push(Token::single_token(TokenType::Slash));
                                }
                            }
                            '&' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::AndEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::And));
                                }
                            }
                            '|' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::OrEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::Or));
                                }
                            }
                            '^' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::CapEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::Cap));
                                }
                            }
                            '~' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::WaveEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::Wave));
                                }
                            }
                            '=' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::EqualEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::Equal));
                                }
                            }
                            '!' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::NotEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::Not));
                                }
                            }
                            '>' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::GreaterEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::Greater));
                                }
                            }
                            '<' => {
                                if chars.c_match('=') {
                                    tokens.push(Token::single_token(TokenType::LessEqual));
                                } else {
                                    tokens.push(Token::single_token(TokenType::Less))
                                }
                            }
                            _ => {
                                return Err(CompilerErrorKind::ScannerError(
                                    chars.line_number,
                                    chars.col_number,
                                    String::from(format!("unrecognized character: '{}'", c)),
                                ))
                            }
                        }
                    }
                }
                None => {
                    tokens.push(Token::single_token(TokenType::Eof));
                    break;
                }
            }
        }
        Ok(tokens)
    }

    fn peek(&mut self, chars: &Peekable<Chars>, position: usize) -> Option<char> {
        // chars.peekable().nth(position)
        None
    }
}
