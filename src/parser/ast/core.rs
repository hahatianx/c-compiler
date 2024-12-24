use std::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};
use crate::parser::operations::Operations;


pub trait AstNode: Debug + Downcast {
    fn get_op(&self) -> Operations;

    fn set_op(&mut self, op: Operations);
}
impl_downcast!(AstNode);

#[derive(Clone, Copy, Debug)]
pub struct AstNodeCore {

    pub op: Operations,

}

impl AstNodeCore {
    pub fn new() -> Self {
        Self { op: Operations::OpNone }
    }
}
