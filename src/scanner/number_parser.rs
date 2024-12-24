use crate::common::errors::error::CompilerErrorKind;

pub struct NumberParser {}

impl NumberParser {

    pub fn new () -> NumberParser {
        Self {}
    }

    pub fn into_checker(&self) -> NumberChecker {
        NumberChecker::new()
    }

}

pub struct NumberChecker {

    cache: String,

}

impl NumberChecker {

    pub fn new() -> NumberChecker {
        Self { cache: String::new() }
    }

    pub fn update(&mut self, c: char) {
        // self.current = self.current * 10 + c as i64 - '0' as i64;
        self.cache.push(c);
    }

    pub fn check(&mut self) -> Result<i64, CompilerErrorKind> {
        // TODO: validate numbers, and support multiple
        // let return_value = Ok(Token::number_token(self.current));
        // self.current = 0;
        // return_value
        let return_value = Ok(self.cache.parse::<i64>().unwrap());
        self.cache.clear();
        return_value
    }

    pub fn can_consume(&self, c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '_' || *c == '.'
    }

}