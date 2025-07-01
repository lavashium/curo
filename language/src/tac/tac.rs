use crate::UnaryKind;

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
        unary_operator: UnaryKind,
        source: TacVal,
        destination: TacVal,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacVal {
    Constant(String),
    Var(String),
}
