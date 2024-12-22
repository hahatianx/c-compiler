use std::fmt::Debug;
use crate::parser::operations::Operations;


pub trait AstNode: Debug {
    fn get_op(&self) -> Operations;

    fn set_op(&mut self, op: Operations);
}

#[derive(Clone, Copy, Debug)]
pub struct AstNodeCore {

    pub op: Operations,

}

impl AstNodeCore {
    pub fn new() -> Self {
        Self { op: Operations::OpNone }
    }
}
