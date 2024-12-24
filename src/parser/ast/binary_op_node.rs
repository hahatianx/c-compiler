use std::fmt::{Debug, Formatter};
use crate::parser::ast::core::AstNodeCore;
use crate::parser::ast::core::AstNode;
use crate::parser::operations::Operations;

pub struct BinaryAstNode {

    core: AstNodeCore,

    pub left: Box<dyn AstNode>,
    pub right: Box<dyn AstNode>,

}

impl BinaryAstNode {
    pub fn new(left: Box<dyn AstNode>, right: Box<dyn AstNode>) -> Self {
        Self {
            core: AstNodeCore::new(),
            left,
            right,
        }
    }
}

impl AstNode for BinaryAstNode {
    fn get_op(&self) -> Operations {
        self.core.op
    }

    fn set_op(&mut self, op: Operations) {
        self.core.op = op;
    }
}

impl Debug for BinaryAstNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {:?} {:?} {:?} ]", self.left, self.get_op(), self.right)
    }
}