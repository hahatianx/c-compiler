use std::fmt::{Debug, Formatter};
use crate::codegen::core::RegLoadable;
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

    pub fn get_value(&self) -> i64 {
        self.value
    }
}

impl RegLoadable for i64 {
    fn to_arm(&self) -> String {
        todo!()
    }

    fn to_gnu_x86(&self) -> String {
        format!("${}", self)
    }
}

impl AstNode for ValueNode {
    fn get_op(&self) -> Operations {
        self.core.op
    }

    fn set_op(&mut self, _op: Operations) {
        todo!()
    }
}

impl Debug for ValueNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " value: {} ", self.value)
    }
}
