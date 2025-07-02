#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmProgram {
    pub function_definition: AsmFunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmFunction {
    pub identifier: String,
    pub instructions: Vec<AsmInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmInstruction {
    Mov {
        src: AsmOperand,
        dst: AsmOperand,
    },
    Unary {
        unary_operator: AsmUnaryOperator,
        operand: AsmOperand,
    },
    Binary {
        binary_operator: AsmBinaryOperator,
        operand1: AsmOperand,
        operand2: AsmOperand,
    },
    Idiv {
        operand: AsmOperand,
    },
    Cdq,
    AllocateStack(i32),
    Ret,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmUnaryOperator {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmBinaryOperator {
    Add,
    Sub,
    Mult
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmOperand {
    Imm(i32),
    Reg(AsmReg),
    Pseudo(String),
    Stack(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmReg {
    AX,
    R10,
    DX,
    R11
}
