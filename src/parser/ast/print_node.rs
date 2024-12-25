use std::fmt::{Debug, Formatter};
use crate::parser::ast::core::{AstNode, AstNodeCore};
use crate::parser::operations::Operations;

pub struct PrintAstNode {

    core: AstNodeCore,

    pub expr: Box<dyn AstNode>,

}

impl PrintAstNode {

    pub fn new(expr: Box<dyn AstNode>) -> Self {
        Self {
            core: AstNodeCore::new(),
            expr,
        }
    }

}

impl Debug for PrintAstNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Print {:?}", self.expr)
    }
}

impl AstNode for PrintAstNode {
    fn get_op(&self) -> Operations {
        self.core.op
    }

    fn set_op(&mut self, op: Operations) {
        self.core.op = op;
    }

}