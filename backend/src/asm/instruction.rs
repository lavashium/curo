use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmProgram {
    function_definition: AsmFunction,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmFunction {
    identifier: String,
    instructions: Vec<AsmInstruction>,
}

#[constructors]
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
        src: AsmOperand,
        dst: AsmOperand,
    },
    Idiv {
        operand: AsmOperand,
    },
    Cdq,
    AllocateStack(i32),
    Ret,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmUnaryOperator {
    Neg,
    Not,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmBinaryOperator {
    Add,
    Sub,
    Mult,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmOperand {
    Imm(i32),
    Reg(AsmReg),
    Pseudo(String),
    Stack(i32),
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmReg {
    AX,
    R10,
    DX,
    R11,
}
