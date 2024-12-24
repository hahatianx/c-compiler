use crate::common::Result;
use crate::codegen::core::CodeGen;
use crate::parser::ast::binary_op_node::BinaryAstNode;
use crate::parser::ast::core::AstNode;
use crate::parser::ast::value_node::ValueNode;
use crate::parser::operations::Operations;

pub struct ASTInterpreter<T>
where T: CodeGen
{
    generator: Box<T>
}

impl<T> ASTInterpreter<T>
where T: CodeGen
{
    pub fn new(generator: Box<T>) -> ASTInterpreter<T> {
        ASTInterpreter { generator }
    }

    pub fn interpret(&mut self, ast: &dyn AstNode) -> Result<()> {
        self.generator.cg_pre_amble()?;
        let reg = self.do_interpret(ast)?;
        self.generator.cg_printreg(reg)?;
        self.generator.cg_post_amble()?;
        Ok(())
    }

    fn do_interpret(&mut self, ast: &dyn AstNode) -> Result<usize>
    {
        let operator = ast.get_op();

        if let Some(binary_node) = ast.downcast_ref::<BinaryAstNode>() {
            let reg1 = self.do_interpret(&*binary_node.left)?;
            let reg2 = self.do_interpret(&*binary_node.right)?;

            match operator {
                Operations::OpIntAdd => {
                    self.generator.cg_add(reg1, reg2)
                },
                Operations::OpIntMinus => {
                    self.generator.cg_sub(reg1, reg2)
                },
                Operations::OpIntMul => {
                    self.generator.cg_mul(reg1, reg2)
                },
                Operations::OpIntDiv => {
                    self.generator.cg_div(reg1, reg2)
                },
                other => unimplemented!("{:?}", other),
            }

        } else if let Some(value_node) = ast.downcast_ref::<ValueNode>() {

            self.generator.cg_load(value_node.get_value())

        } else {
            panic!("Unknown node type");
        }
    }
}