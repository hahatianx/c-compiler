use crate::parser::ast::binary_op_node::BinaryAstNode;
use crate::parser::ast::core::AstNode;
use crate::parser::operations::Operations;
use crate::parser::parser::Parser;
use crate::scanner::tokens::{TokenType};
use crate::common::Result;
use crate::parser::ast::value_node::ValueNode;
use crate::parser::precedence::Precedence;

pub type ParseFunc = fn(&mut Parser, bool) -> Result<Box<dyn AstNode>>;

pub fn not_implemented_parser<'a>(_parser: &'a mut Parser, _v: bool) -> Result<Box<dyn AstNode>> {
    unimplemented!()
}


pub fn binary<'a>(parser: &'a mut Parser, _can_assign: bool) -> Result<Box<dyn AstNode>> {
    let operator_type = parser.get_previous().get_token_type();
    let rule = operator_type.get_rule();

    let left_hand = parser.pop_ast();
    let next_precedence = Precedence::try_from(rule.precedence.value() + 1).unwrap();
    let right_hand = parser.parse_precedence(next_precedence)?;

    let mut node = BinaryAstNode::new(left_hand, right_hand);

    match operator_type {
        TokenType::Plus => node.set_op(Operations::OpIntAdd),
        TokenType::Minus => node.set_op(Operations::OpIntMinus),
        TokenType::Star => node.set_op(Operations::OpIntMul),
        TokenType::Slash => node.set_op(Operations::OpIntDiv),
        others => unimplemented!("{:?}", others),
    }

    Ok(Box::new(node))
}

pub fn grouping<'a>(parser: &'a mut Parser, _can_assign: bool) -> Result<Box<dyn AstNode>> {
    let expression = parser.expression()?;
    parser.consume(TokenType::RightParen, "Expect ')' after expression.")?;
    Ok(expression)
}

pub fn int_number<'a>(parser: &'a mut Parser, _can_assign: bool) -> Result<Box<dyn AstNode>> {
    let token = parser.get_previous();
    Ok(Box::new(ValueNode::new(token.get_value() as i64)))
}

