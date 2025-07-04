use crate::{BinaryKind, UnaryKind};
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TacProgram {
    function_definition: TacFunction,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TacFunction {
    identifier: String,
    instructions: Vec<TacInstruction>,
}

#[constructors]
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

#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacVal {
    Constant(String),
    Var(String),
}
