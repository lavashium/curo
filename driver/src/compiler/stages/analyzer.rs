use crate::compiler::*;
use common::*;
use language::*;

use super::*;

pub struct AnalysisStage;

impl Factory<PipelineResult<AstProgram>, AstProgram, PipelineContext<'_, '_>> for AnalysisStage {
    fn run(program: &mut AstProgram, ctx: &mut PipelineContext) -> PipelineResult<AstProgram> {
        todo!()
    }
}