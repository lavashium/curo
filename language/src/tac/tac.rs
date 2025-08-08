use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TacProgram {
    top_level: Vec<TacTopLevel>,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacTopLevel {
    Function {
        identifier: String,
        global: bool,
        params: Vec<String>,
        instructions: Vec<TacInstruction>,
    },

    StaticVariable {
        identifier: String,
        global: bool,
        init: String
    }
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TacFunction {
    identifier: String,
    params: Vec<String>,
    instructions: Vec<TacInstruction>,
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacInstruction {
    Return {
        val: TacVal,
    },
    Unary {
        operator: TacUnaryKind,
        source: TacVal,
        destination: TacVal,
    },
    Binary {
        operator: TacBinaryKind,
        source1: TacVal,
        source2: TacVal,
        destination: TacVal,
    },
    Copy {
        src: TacVal,
        dst: TacVal,
    },
    Jump {
        target: String,
    },
    JumpIfZero {
        condition: TacVal,
        target: String,
    },
    JumpIfNotZero {
        condition: TacVal,
        target: String,
    },
    Label(String),
    FunCall {
        fun_name: String,
        args: Vec<TacVal>,
        dst: TacVal,
    }
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacVal {
    Constant(String),
    Var(String),
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacUnaryKind {
    Complement,
    Negate,
    Not
}

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacBinaryKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}