use crate::common::Result;

pub trait RegLoadable {
    fn to_arm(&self) -> String;

    fn to_gnu_x86(&self) -> String;
}

pub trait CodeGen {

    fn cg_pre_amble(&mut self) -> Result<()>;

    fn cg_post_amble(&mut self) -> Result<()>;

    fn cg_push(&mut self, reg: usize) -> Result<()>;

    fn cg_pop(&mut self, reg: usize) -> Result<()>;

    fn cg_load<T: RegLoadable>(&mut self, value: T) -> Result<usize>;

    fn cg_add(&mut self, reg1: usize, reg2: usize) -> Result<usize>;

    fn cg_sub(&mut self, reg1: usize, reg2: usize) -> Result<usize>;

    fn cg_mul(&mut self, reg1: usize, reg2: usize) -> Result<usize>;

    fn cg_div(&mut self, reg1: usize, reg2: usize) -> Result<usize>;

    fn cg_printreg(&mut self, reg: usize) -> Result<()>;

}