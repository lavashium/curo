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
            AsmOperand::Data(name) => format!("{}(%rip)", name),
            AsmOperand::Imm(value) => format!("${}", value),
            AsmOperand::Pseudo(_) => panic!("Pseudo operand should not exist at emission"),
        }
    }
}

impl AsmOperand {
    pub fn to_8byte(&self) -> String {
        match &self {
            AsmOperand::Reg(reg) => reg.to_asm_8byte(),
            _ => self.to_asm(),
        }
    }

    pub fn to_4byte(&self) -> String {
        match &self {
            AsmOperand::Reg(reg) => reg.to_asm_4byte(),
            _ => self.to_asm(),
        }
    }

    pub fn to_1byte(&self) -> String {
        match &self {
            AsmOperand::Reg(reg) => reg.to_asm_1byte(),
            _ => self.to_asm(),
        }
    }
}

impl AsmReg {
    pub fn to_asm_8byte(&self) -> String {
        match self {
            AsmReg::AX => "%rax",
            AsmReg::CX => "%rcx",
            AsmReg::DX => "%rdx",
            AsmReg::SI => "%rsi",
            AsmReg::DI => "%rdi",
            AsmReg::R8  => "%r8",
            AsmReg::R9  => "%r9",
            AsmReg::R10 => "%r10",
            AsmReg::R11 => "%r11",
        }
        .to_string()
    }
    
    pub fn to_asm_4byte(&self) -> String {
        match self {
            AsmReg::AX => "%eax",
            AsmReg::CX => "%ecx",
            AsmReg::DX => "%edx",
            AsmReg::SI => "%esi",
            AsmReg::DI => "%edi",
            AsmReg::R8  => "%r8d",
            AsmReg::R9  => "%r9d",
            AsmReg::R10 => "%r10d",
            AsmReg::R11 => "%r11d",
        }
        .to_string()
    }
    
    pub fn to_asm_1byte(&self) -> String {
        match self {
            AsmReg::AX => "%al",
            AsmReg::CX => "%cl",
            AsmReg::DX => "%dl",
            AsmReg::SI => "%sil",
            AsmReg::DI => "%dil",
            AsmReg::R8  => "%r8b",
            AsmReg::R9  => "%r9b",
            AsmReg::R10 => "%r10b",
            AsmReg::R11 => "%r11b",
        }
        .to_string()
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
