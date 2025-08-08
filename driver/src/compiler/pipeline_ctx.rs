use common::*;
use constructors::constructors;

use crate::*;

#[constructors]
pub struct PipelineContext<'scp, 'ctx> {
    pub ctx: &'scp mut CompilerContext<'ctx>,
    pub stage: PipelineStage,
    pub debug: String,
    pub errcode: ErrCode,
}

impl Context for PipelineContext<'_, '_> {}

#[constructors]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PipelineStage {
    Lexer,
    Parser,
    Analysis,
    TacGeneration,
    AssemblyGeneration,
    AssemblyAllocation,
    AssemblyLegalization,
    CodeEmission,
}