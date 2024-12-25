use num_enum::TryFromPrimitive;
use crate::parser::parse_func::{binary, grouping, int_number, not_implemented_parser, ParseFunc};
use crate::scanner::tokens::TokenType;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Precedence {

    PrecNone = 0,
    PrecAssignment,
    PrecOr,
    PrecAnd,
    PrecEquality,
    PrecComparison,
    PrecTerm,
    PrecFactor,
    PrecBitOr,
    PrecBitAnd,
    PrecUnary,
    PrecCall,
    PrecPrimary,

    /**
    This marks the end of enum Precedence, ensuring prec + 1 never hits `None`
    */
    PrecEnd,

}

impl Precedence {
    pub fn value(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy)]
pub struct ParseRule {

    prefix: ParseFunc,
    infix: ParseFunc,
    pub precedence: Precedence,

}

impl ParseRule {

    pub fn get_prefix(&self) -> Option<ParseFunc> {
        if self.prefix == not_implemented_parser {
            None
        } else {
            Some(self.prefix)
        }
    }

    pub fn get_infix(&self) -> Option<ParseFunc> {
        if self.infix == not_implemented_parser {
            None
        } else {
            Some(self.infix)
        }
    }
}

impl TokenType {

    pub fn get_rule(&self) -> ParseRule {
        match self {
            TokenType::LeftParen => PARSE_RULE_LEFT_PARAN,
            TokenType::RightParen => PARSE_RULE_RIGHT_PARAN,
            TokenType::Plus => PARSE_RULE_PLUS,
            TokenType::Minus => PARSE_RULE_MINUS,
            TokenType::Star => PARSE_RULE_STAR,
            TokenType::Slash => PARSE_RULE_SLASH,
            TokenType::Integer => PARSE_RULE_INTEGER,

            TokenType::LeftBrace => PARSE_RULE_LEFT_BRACE,
            TokenType::RightBrace => PARSE_RULE_RIGHT_BRACE,

            TokenType::Semicolon => PARSE_RULE_SEMICOLON,
            TokenType::Eof => PARSE_RULE_EOF,
            others=> unimplemented!("{:?}", others),
        }
    }

}

static PARSE_RULE_LEFT_PARAN: ParseRule = ParseRule {
    prefix: grouping,
    infix: not_implemented_parser,
    precedence: Precedence::PrecCall,
};

static PARSE_RULE_RIGHT_PARAN: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_LEFT_BRACE: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_RIGHT_BRACE: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_LEFT_SQUARE: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_RIGHT_SQUARE: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_COMMA: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_DOT: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_MINUS: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: binary,
    precedence: Precedence::PrecTerm,
};

static PARSE_RULE_PLUS: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: binary,
    precedence: Precedence::PrecTerm,
};

static PARSE_RULE_SEMICOLON: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_SLASH: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: binary,
    precedence: Precedence::PrecFactor,
};

static PARSE_RULE_STAR: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: binary,
    precedence: Precedence::PrecFactor,
};

static PARSE_RULE_PERCENT: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_BANG: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_BANG_EQUAL: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_EQUAL: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_EQUAL_EQUAL: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_GREATER: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_GREATER_EQUAL: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_LESS: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_LESS_EQUAL: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_AMPERSAND: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_CAP: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_LEFT_ARROW: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_RIGHT_ARROW: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_IDENTIFIER: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_STRING: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_INTEGER: ParseRule = ParseRule {
    prefix: int_number,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};

static PARSE_RULE_EOF: ParseRule = ParseRule {
    prefix: not_implemented_parser,
    infix: not_implemented_parser,
    precedence: Precedence::PrecNone,
};