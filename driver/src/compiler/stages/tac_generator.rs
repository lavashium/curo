use crate::compiler::*;
use common::*;
use language::*;
use middleend::*;

use super::*;

pub struct TacGeneratorStage;

impl Factory<PipelineResult<TacProgram>, TypedProgram, PipelineContext<'_, '_>> for TacGeneratorStage {
    fn run(program: &mut TypedProgram, ctx: &mut PipelineContext) -> PipelineResult<TacProgram> {
        let mut tac_ctx = TacGenContext::new(
            ctx.ctx
        );

        let mut generator = TacGenerator::new(program);
        let program = generator.generate(&mut tac_ctx);

        if ctx.stage == PipelineStage::TacGeneration {
            let debug = format!("{:#?}", program);
            return PipelineResult::Halt(Ok(debug));
        }

        PipelineResult::Continue(program)
    }
}