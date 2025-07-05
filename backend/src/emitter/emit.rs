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
            AsmOperand::Reg(reg) => reg.to_asm(),
            AsmOperand::Stack(offset) => format!("{}(%rbp)", offset),
            AsmOperand::Imm(value) => format!("${}", value),
            AsmOperand::Pseudo(_) => panic!("Pseudo operand should not exist at emission"),
        }
    }
}

pub fn set_c_c_adjuster(op: &AsmOperand) -> String {
    match op {
        AsmOperand::Reg(reg) => reg.to_asm_1byte(),
        some => some.to_asm(),
    }
}

impl AsmReg {
    pub fn to_asm_4byte(&self) -> String {
        match self {
            AsmReg::AX => "%eax".to_string(),
            AsmReg::DX => "%edx".to_string(),
            AsmReg::R10 => "%r10d".to_string(),
            AsmReg::R11 => "%r11d".to_string(),
        }
    }
    
    pub fn to_asm_1byte(&self) -> String {
        match self {
            AsmReg::AX => "%al".to_string(),
            AsmReg::DX => "%dl".to_string(),
            AsmReg::R10 => "%r10b".to_string(),
            AsmReg::R11 => "%r11b".to_string(),
        }
    }
}

impl ToAsm for AsmReg {
    fn to_asm(&self) -> String {
        self.to_asm_4byte()
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

impl ToAsm for AsmCondCode {
    fn to_asm(&self) -> String {
        match self {
            AsmCondCode::E  => "e",
            AsmCondCode::NE => "ne",
            AsmCondCode::L  => "l",
            AsmCondCode::LE => "le",
            AsmCondCode::G  => "g",
            AsmCondCode::GE => "ge",
        }
        .to_string()
    }
}
