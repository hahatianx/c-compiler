use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq, Copy)]
pub enum Operations {

    OpNone = 0,

    OpIntAdd,
    OpIntMinus,
    OpIntMul,
    OpIntDiv,

    OpValueInt,

}

impl Debug for Operations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::OpNone => write!(f, "OpNone"),
            Operations::OpIntAdd => write!(f, "+"),
            Operations::OpIntMinus => write!(f, "-"),
            Operations::OpIntMul => write!(f, "*"),
            Operations::OpIntDiv => write!(f, "/"),
            _ => unimplemented!(),
        }
    }
}