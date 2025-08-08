use std::marker::PhantomData;
use common::*;
use language::*;
use middleend::*;

use super::*;

pub struct TacGeneratorStage<'scp, 'ctx> {
    _driver: PhantomData<PipelineContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for TacGeneratorStage<'scp, 'ctx> {
    type Context = PipelineContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<PipelineResult<TacProgram>, TypedProgram> for TacGeneratorStage<'scp, 'ctx> {
    fn run(program: &mut TypedProgram, ctx: &mut PipelineContext<'scp, 'ctx>) -> PipelineResult<TacProgram> {
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