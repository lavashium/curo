#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmProgram {
    pub function_definition: AsmFunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmFunction {
    pub name: String,
    pub instructions: Vec<AsmInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmInstruction {
    Mov {
        source: AsmOperand,
        dest: AsmOperand,
    },
    Ret
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmOperand {
    Imm(i32),
    Register
}