use crate::common::errors::error::CompilerErrorKind;
use crate::common::Result;
use crate::scanner::number_parser::{NumberParser};
use crate::scanner::tokens::{Token, TokenType};
use crate::utils::trie::KeywordTrie;
use std::iter::Peekable;
use std::str::Chars;
use crate::source_code::SourceCode;

pub struct Scanner<'a> {
    keyword_trie: KeywordTrie,
    number_parser: NumberParser,
    chars: ScannerPeekable<'a>,
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

impl<'a> Scanner<'a> {
    pub fn new(source_code: &'a SourceCode) -> Self {
        Self {
            keyword_trie: KeywordTrie::new(),
            number_parser: NumberParser::new(),
            chars: ScannerPeekable::new(source_code.get_source_code().chars().peekable()),
        }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.chars.line_number, self.chars.col_number)
    }

    pub fn scan(&mut self) -> Result<Token> {

        let mut block_comment = 0;
        let mut line_comment = 0;

        let mut keyword_checker = self.keyword_trie.into_checker();
        let mut number_checker = self.number_parser.into_checker();
        loop {
            match self.chars.next() {
                Some(c) => {

                    if line_comment > 0 {
                        if c == '\n' {
                            line_comment = 0;
                        }
                        continue;
                    }
                    if block_comment > 0 {
                        if c == '*' && self.chars.c_match('/') {
                            block_comment -= 1;
                        }
                        continue;
                    }

                    if c.is_whitespace() {
                    } else if c.is_ascii_alphabetic() || c == '_' {
                        // allowed chars [a-zA-Z_]{1}[a-zA-Z_0-9]*
                        keyword_checker.update(c);
                        while let Some(next_chars) = self.chars.peek() {
                            if !keyword_checker.can_consume(next_chars) {
                                break;
                            }
                            keyword_checker.update(self.chars.next().unwrap());
                        }
                        if let Some(keyword) = keyword_checker.check() {
                            Token::single_token(keyword);
                        } else {
                            let str = keyword_checker.get_str()?;
                            return Ok(Token::Identifier(str));
                        }
                    } else if c.is_digit(10) {
                        number_checker.update(c);
                        // allowed chars [0-9]{1}[0-9a-z.]*, delegate to number checker for validation checks
                        while let Some(next_chars) = self.chars.peek() {
                            if !number_checker.can_consume(next_chars) {
                                break;
                            }
                            number_checker.update(self.chars.next().unwrap());
                        }
                        if let Ok(value) = number_checker.check() {
                            // a valid token
                            return Ok(Token::number_token(value));
                        } else {
                            // TODO: raise number parse errors
                        }
                    } else if c == '"' || c == '\'' {
                        // TODO: string constants
                        let str = parse_constant_chars(&mut self.chars, c)?;
                        match c {
                            '\'' => return Ok(Token::text_token(TokenType::Char, &str)),
                            '"' => return Ok(Token::text_token(TokenType::String, &str)),
                            _ => {panic!("This branch should not happen!")}
                        }
                    } else {
                        match c {
                            '#' => return Ok(Token::single_token(TokenType::Hash)),
                            '(' => return Ok(Token::single_token(TokenType::LeftParen)),
                            ')' => return Ok(Token::single_token(TokenType::RightParen)),
                            '{' => return Ok(Token::single_token(TokenType::LeftBrace)),
                            '}' => return Ok(Token::single_token(TokenType::RightBrace)),
                            ',' => return Ok(Token::single_token(TokenType::Comma)),
                            ';' => return Ok(Token::single_token(TokenType::Semicolon)),
                            '+' => {
                                return if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::PlusEqual))
                                } else {
                                    Ok(Token::single_token(TokenType::Plus))
                                }
                            }
                            '-' => {
                                return if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::MinusEqual))
                                } else {
                                    Ok(Token::single_token(TokenType::Minus))
                                }
                            }
                            '*' => {
                                return if self.chars.c_match('/') {
                                    Err(CompilerErrorKind::ScannerError(
                                        self.chars.line_number,
                                        self.chars.col_number,
                                        String::from("unrecognized end of comment '*/'")
                                    ))
                                } else if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::StarEqual))
                                } else {
                                    Ok(Token::single_token(TokenType::Star))
                                }
                            }
                            '/' => {
                                if self.chars.c_match('=') {
                                    return Ok(Token::single_token(TokenType::SlashEqual));
                                } else if self.chars.c_match('/') {
                                    line_comment += 1;
                                } else if self.chars.c_match('*'){
                                    block_comment += 1;
                                } else {
                                    return Ok(Token::single_token(TokenType::Slash));
                                }
                            }
                            '&' => {
                                return if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::AndEqual))
                                } else {
                                    Ok(Token::single_token(TokenType::And))
                                }
                            }
                            '|' => {
                                return if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::OrEqual))
                                } else {
                                    Ok(Token::single_token(TokenType::Or))
                                }
                            }
                            '^' => {
                                return if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::CapEqual))
                                } else {
                                    Ok(Token::single_token(TokenType::Cap))
                                }
                            }
                            '~' => {
                                return if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::WaveEqual))
                                } else {
                                    Ok(Token::single_token(TokenType::Wave))
                                }
                            }
                            '=' => {
                                return if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::EqualEqual))
                                } else {
                                    Ok(Token::single_token(TokenType::Equal))
                                }
                            }
                            '!' => {
                                return if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::NotEqual))
                                } else {
                                    Ok(Token::single_token(TokenType::Not))
                                }
                            }
                            '>' => {
                                return if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::GreaterEqual))
                                } else if self.chars.c_match('>') {
                                    Ok(Token::single_token(TokenType::RightArrow))
                                } else {
                                    Ok(Token::single_token(TokenType::Greater))
                                }
                            }
                            '<' => {
                                return if self.chars.c_match('=') {
                                    Ok(Token::single_token(TokenType::LessEqual))
                                } else if self.chars.c_match('<') {
                                    Ok(Token::single_token(TokenType::LeftArrow))
                                } else {
                                    Ok(Token::single_token(TokenType::Less))
                                }
                            }
                            _ => {
                                return Err(CompilerErrorKind::ScannerError(
                                    self.chars.line_number,
                                    self.chars.col_number,
                                    String::from(format!("unrecognized character: '{}'", c)),
                                ))
                            }
                        }
                    }
                }
                None => {
                    return Ok(Token::single_token(TokenType::Eof));
                }
            }
        }
    }
}
