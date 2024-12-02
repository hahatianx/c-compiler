

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,

    Integer,
    Double,

    Eof,
    None,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,

    // type keywords,
    T_Int,
    T_Double,
    T_Float,
    T_String,

    // Keywords
    Break,
    Continue,
    Return,
    If,
    Else,
    For,
    While,


}


#[derive(Clone, Debug, PartialEq, Copy)]
union TokenValue {
    float_value: f64,
    int_value: i64,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,

    text: Option<&'a str>,

    value: Option<TokenValue>,

}

impl<'a> Token<'a> {

    pub fn EOF() -> Self {
        Self {
            token_type: TokenType::Eof,
            text: None,
            value: None,
        }
    }

    pub fn new(token_type: TokenType, text: &'a str, value: TokenValue) -> Self {
        Self {
            token_type,
            text: Some(text),
            value: Some(value),
        }
    }

    pub fn get_value<T>(&self) -> Option<T>
    where T: Copy
    {
        unsafe {
            match self.token_type {
                TokenType::Integer => Some(self.value.unwrap().int_value),
                TokenType::Double => Some(self.value.unwrap().float_value),
                _ => None
            }
        }
    }

}