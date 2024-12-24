use crate::common::errors::error::CompilerErrorKind;
use crate::common::Result;

pub enum RegistersType {
    X86,
    ARM,
}

pub struct Registers {
    avail: [bool; 4],
    name: [&'static str; 4],
}

impl Registers {

    pub fn new(reg_type: RegistersType) -> Registers {
        Self {
            avail: [true; 4],
            name: match reg_type {
                RegistersType::X86 => X86_REGISTERS,
                RegistersType::ARM => ARM_REGISTERS,
            },
        }
    }

    pub fn init_registers(&mut self) {
        for i in 0..4 {
            self.avail[i] = true;
        }
    }

    pub fn allocate_register(&mut self) -> Result<usize> {
        for i in 0..4 {
            if self.avail[i] {
                self.avail[i] = false;
                return Ok(i);
            }
        }
        Err(CompilerErrorKind::CodeGeneratorError(String::from("No registers available")))
    }

    pub fn free_register(&mut self, reg: usize) {
        self.avail[reg] = true;
    }

    pub fn register_name(&self, reg: usize) -> &str {
        self.name[reg]
    }

}

static X86_REGISTERS: [&'static str; 4] = [
    "%r8", "%r9", "%r10", "%r11",
];

static ARM_REGISTERS: [&'static str; 4] = [
    "r0", "r1", "r2", "r3",
];