use crate::asm::*;

pub trait ToAsm {
    fn to_asm(&self) -> String;
}

impl<T: ToString> ToAsm for T {
    fn to_asm(&self) -> String {
        self.to_string()
    }
}

impl ToAsm for AsmOperand {
    fn to_asm(&self) -> String {
        match self {
            AsmOperand::Reg(reg) =>      reg.to_asm(),
            AsmOperand::Stack(offset) => format!("{}(%rbp)", offset),
            AsmOperand::Imm(value) =>    format!("${}", value),
            AsmOperand::Pseudo(_) =>     panic!("Pseudo operand should not exist at emission"),
        }
    }
}

impl ToAsm for AsmReg {
    fn to_asm(&self) -> String {
        match self {
            AsmReg::AX =>  "%eax",
            AsmReg::DX =>  "%edx",
            AsmReg::R10 => "%r10d",
            AsmReg::R11 => "%r11d",
        }
        .to_string()
    }
}

impl ToAsm for AsmBinaryOperator {
    fn to_asm(&self) -> String {
        match self {
            AsmBinaryOperator::Add => "addl",
            AsmBinaryOperator::Sub => "subl",
            AsmBinaryOperator::Mult => "imull",
        }
        .to_string()
    }
}

impl ToAsm for AsmUnaryOperator {
    fn to_asm(&self) -> String {
        match self {
            AsmUnaryOperator::Neg => "negl",
            AsmUnaryOperator::Not => "notl",
        }
        .to_string()
    }
}
