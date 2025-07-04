use crate::{BinaryKind, UnaryKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TacProgram {
    pub function_definition: TacFunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TacFunction {
    pub identifier: String,
    pub instructions: Vec<TacInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacInstruction {
    Return {
        val: TacVal,
    },
    Unary {
        operator: UnaryKind,
        source: TacVal,
        destination: TacVal,
    },
    Binary {
        operator: BinaryKind,
        source1: TacVal,
        source2: TacVal,
        destination: TacVal,
    },
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacVal {
    Constant(String),
    Var(String),
}
