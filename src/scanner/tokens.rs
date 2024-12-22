use std::fmt::{Debug, Formatter};

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    LeftArrow,
    RightArrow,

    Identifier,
    Char,
    String,
    Integer,
    Double,

    Eof,
    None,

    // Operators
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    And,
    AndEqual,
    Or,
    OrEqual,
    Cap,
    CapEqual,
    Wave,
    WaveEqual,
    Not,
    Hash,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Equal,
    EqualEqual,
    NotEqual,

    // type keywords,
    KeyInt,
    KeyLong,
    KeyDouble,
    KeyFloat,
    KeyString,

    // Keywords
    Break,
    Continue,
    Return,
    If,
    Else,
    For,
    While,
}


#[derive(Clone, Copy)]
#[repr(C)]
union TokenValue {
    pub float_value: f64,
    pub int_value: i64,
}

impl Debug for TokenValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { self.int_value })
    }
}

#[derive(Clone)]
pub enum Token {
    None,
    Single(TokenType),
    Identifier(String),
    Number(TokenType, TokenValue),
    Text(TokenType, String),
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Single(tt) => write!(f, "[{:?}]", tt),
            Token::Identifier(ident) => write!(f, "[Identifier: {}]", ident),
            Token::Number(tt, value) => {
                match tt {
                    TokenType::Integer => write!(f, "[Int: {:?}]", value),
                    TokenType::Double => write!(f, "[Double: {:?}]", value),
                    _ => panic!("Should not happen"),
                }
            },
            Token::Text(tt, value) => {
                match tt {
                    TokenType::String => write!(f, "[Str: {:?}]", value),
                    TokenType::Char => write!(f, "[Char: {:?}]", value),
                    _ => panic!("Should not happen!"),
                }
            },
            _ => {write!(f, "")},
        }
    }
}

impl<'a> Token {
    pub fn single_token(token_type: TokenType) -> Self {
        Self::Single(token_type)
    }

    pub fn number_token(value: i64) -> Self {
        Self::Number(TokenType::Integer, TokenValue { int_value: value })
    }

    pub fn text_token(token_type: TokenType, value: &'a str) -> Self {
        // token_type must be either String or Char
        Self::Text(token_type, value.to_string())
    }

    pub fn identifier(text: &'a str) -> Self {
        // identifiers
        Self::Identifier(text.to_string())
    }

    pub fn get_token_type(&self) -> TokenType {
        match self {
            Token::Single(t) => *t,
            Token::Identifier(_) => TokenType::Identifier,
            Token::Number(t, _) => *t,
            Token::Text(t, _) => *t,
            Token::None => TokenType::None,
        }
    }
}