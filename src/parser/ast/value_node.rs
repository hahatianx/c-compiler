use std::fmt::{Debug, Formatter};
use crate::parser::ast::core::{AstNode, AstNodeCore};
use crate::parser::operations::Operations;

pub struct ValueNode {

    core: AstNodeCore,

    value: i64,
}

impl ValueNode {
    pub fn new(value: i64) -> Self {
        Self {
            core: AstNodeCore::new(),
            value,
        }
    }
}

impl AstNode for ValueNode {
    fn get_op(&self) -> Operations {
        self.core.op
    }

    fn set_op(&mut self, op: Operations) {
        todo!()
    }
}

impl Debug for ValueNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " value: {} ", self.value)
    }
}
