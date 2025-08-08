use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmProgram {
    top_level: Vec<AsmTopLevel>,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmTopLevel {
    Function {
        identifier: String,
        global: bool,
        instructions: Vec<AsmInstruction>,
        stack_size: i32,
    },
    StaticVariable {
        identifier: String,
        global: bool,
        init: String,
    }
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
    Cmp {
        operand1: AsmOperand,
        operand2: AsmOperand,
    },
    Idiv {
        operand: AsmOperand,
    },
    Cdq,
    Jmp(String),
    JmpCC {
        cond: AsmCondCode,
        label: String,
    },
    SetCC {
        cond: AsmCondCode,
        operand: AsmOperand,
    },
    Label(String),
    AllocateStack(i32),
    DeallocateStack(i32),
    Push(AsmOperand),
    Call(String),
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
    Mult,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmOperand {
    Imm(i32),
    Reg(AsmReg),
    Pseudo(String),
    Stack(i32),
    Data(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmCondCode {
    E,
    NE,
    G,
    GE,
    L,
    LE
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmReg {
    AX,
    CX,
    DX,
    DI,
    SI,
    R8,
    R9,
    R10,
    R11,
}
