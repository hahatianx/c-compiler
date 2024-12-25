use std::fmt::{Debug, Formatter};
use crate::parser::ast::core::{AstNode, AstNodeCore};
use crate::parser::operations::Operations;

pub struct BlockNode {

    core: AstNodeCore,

    pub block: Vec<Box<dyn AstNode>>,

}

impl Debug for BlockNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statements: {{\n")?;
        for ast_node in self.block.iter() {
            write!(f, "{:?}\n", ast_node)?;
        }
        write!(f, "}}\n")?;
        Ok(())
    }
}

impl AstNode for BlockNode {
    fn get_op(&self) -> Operations {
        self.core.op
    }

    fn set_op(&mut self, _op: Operations) {
        unimplemented!()
    }
}

impl BlockNode {
    pub fn new(block: Vec<Box<dyn AstNode>>) -> Self {
        Self {
            core: AstNodeCore::new(),
            block,
        }
    }
}