use std::str::Chars;
use crate::scanner::tokens::Token;
use crate::utils::trie::{KeywordTrie};
use crate::common::Result;

struct Scanner<'a> {

    source_code: &'a str,
    keyword_trie: KeywordTrie,

    chars: Chars<'_>,

    line_number: usize,
}

impl Scanner {

    fn new(code: String) -> Self {
        Self {
            keyword_trie: KeywordTrie::new(),
            source_code: code.as_str(),
            chars: code.chars(),
            line_number: 0,
        }
    }

    fn scan(&mut self) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        // let mut comment_count = 0;
        let mut keyword_checker = self.keyword_trie.into_checker();
        loop {
            match self.chars.next() {
                Some(c) => {
                    if c.is_whitespace() {
                        // keyword_checker
                        if c == '\n' {
                            self.line_number += 1
                        }
                    } else if c.is_alphabetic() {
                        keyword_checker.update(c);
                    } else if c.is_digit(10) {

                    } else {
                        match c {
                            '+' => tokens.push(),
                            '-' => tokens.push(),
                            '*' => tokens.push(),
                            '/' => tokens.push(),

                        }
                    }
                }
                None => {
                    tokens.push(Token::EOF());
                    break;
                }
            }
        }
        Ok(tokens)
    }

    fn peek(&mut self, position: usize) -> Option<char> {
        None
    }
}

