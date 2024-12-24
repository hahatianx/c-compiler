use crate::codegen::core::{CodeGen, RegLoadable};
use crate::codegen::module::output::FileOutput;
use crate::codegen::module::registers::{Registers, RegistersType};
use crate::file_writeln;

pub struct X86Generator<'a> {
    registers: Registers,
    output: &'a mut FileOutput,
}

impl<'a> X86Generator<'a> {
    pub fn new(output: &mut FileOutput) -> X86Generator {
        X86Generator {
            registers: Registers::new(RegistersType::X86),
            output,
        }
    }
}

impl CodeGen for X86Generator<'_> {
    fn cg_pre_amble(&mut self) -> crate::common::Result<()> {
        self.registers.init_registers();

        file_writeln!(self, output, writeln,
            ".text",
            ".LC0:",
            "\t.string\t\"%d\\n\"",
            "printint:",
            "\tpushq\t%rbp",
            "\tmovq\t%rsp, %rbp",
            "\tsubq\t$16, %rsp",
            "\tmovl\t%edi, -4(%rbp)",
            "\tmovl\t-4(%rbp), %eax",
            "\tmovl\t%eax, %esi",
            "\tleaq\t.LC0(%rip), %rdi",
            "\tmovl\t$0, %eax",
            "\tcall\tprintf@PLT",
            "\tnop",
            "\tleave",
            "\tret",
            "",
            ".globl\tmain",
            ".type\tmain, @function",
            "main:",
            "\tpushq\t%rbp",
            "\tmovq\t%rsp, %rbp",
        );

        Ok(())
    }

    fn cg_post_amble(&mut self) -> crate::common::Result<()> {
        /**
            return 0;
            iret -> exit(main());
        */
        file_writeln!(self, output, writeln,
            "\tmovl\t$0, %eax",
            "\tpopq\t$rbp",
            "\tret",
        );

        Ok(())
    }

    fn cg_push(&mut self, reg: usize) -> crate::common::Result<()> {
        todo!()
    }

    fn cg_pop(&mut self, reg: usize) -> crate::common::Result<()> {
        todo!()
    }

    fn cg_load<T: RegLoadable>(&mut self, value: T) -> crate::common::Result<usize> {
        let reg = self.registers.allocate_register()?;

        self.output.writeln(&format!(
            "\tmovq\t{}, {}",
            value.to_gnu_x86(),
            self.registers.register_name(reg)
        ))?;

        Ok(reg)
    }

    fn cg_add(&mut self, reg1: usize, reg2: usize) -> crate::common::Result<usize> {
        self.output.writeln(&format!(
            "\taddq\t{}, {}",
            self.registers.register_name(reg1),
            self.registers.register_name(reg2)
        ))?;
        self.registers.free_register(reg1);

        Ok(reg2)
    }

    fn cg_sub(&mut self, reg1: usize, reg2: usize) -> crate::common::Result<usize> {
        self.output.writeln(&format!(
            "\tsubq\t{}, {}",
            self.registers.register_name(reg2),
            self.registers.register_name(reg1)
        ))?;
        self.registers.free_register(reg2);

        Ok(reg1)
    }

    fn cg_mul(&mut self, reg1: usize, reg2: usize) -> crate::common::Result<usize> {
        self.output.writeln(&format!(
            "\timulq\t{}, {}",
            self.registers.register_name(reg1),
            self.registers.register_name(reg2)
        ))?;
        self.registers.free_register(reg1);

        Ok(reg2)
    }

    fn cg_div(&mut self, reg1: usize, reg2: usize) -> crate::common::Result<usize> {
        self.output.writeln(&format!(
            "\tmovq\t{}, %rax",
            self.registers.register_name(reg1)
        ))?;
        file_writeln!(self, output, writeln,
            "\tcqo");
        self.output.writeln(&format!(
            "\tidivq\t{}",
            self.registers.register_name(reg2)
        ))?;
        self.output.writeln(&format!(
            "\tmovq\t%rax, {}",
            self.registers.register_name(reg1)
        ))?;
        self.registers.free_register(reg2);

        Ok(reg1)
    }

    fn cg_printreg(&mut self, reg: usize) -> crate::common::Result<()> {
        self.output.writeln(&format!(
            "\tmovq\t{}, %rdi",
            self.registers.register_name(reg)
        ))?;
        file_writeln!(self, output, writeln, "\tcall\tprintint");

        self.registers.free_register(reg);
        Ok(())
    }
}
