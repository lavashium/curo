use std::collections::HashMap;
use std::marker::PhantomData;
use common::*;
use language::*;
use backend::*;

use super::*;
pub struct X86_64<'scp, 'ctx> {
    _driver: PhantomData<PipelineContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for X86_64<'scp, 'ctx> {
    type Context = PipelineContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<PipelineResult<String>, TacProgram> for X86_64<'scp, 'ctx> {
    fn run(program: &mut TacProgram, ctx: &mut PipelineContext<'scp, 'ctx>) -> PipelineResult<String> {
        let mut generator_ctx = GeneratorContext::new(
            ctx.ctx
        );

        let mut generator = AsmGenerator::new(program);
        let mut program = generator.generate(&mut generator_ctx);

        if ctx.stage == PipelineStage::AssemblyGeneration {
            let debug = format!("{:#?}", program);
            return PipelineResult::Halt(Ok(debug));
        }

        let mut allocator_ctx = AllocatorContext::new(
            ctx.ctx,
            HashMap::new(),
            0
        );

        let mut allocator = AsmAllocator::new(&mut program);
        allocator.allocate(&mut allocator_ctx);

        if ctx.stage == PipelineStage::AssemblyAllocation {
            let debug = format!("{:#?}", program);
            return PipelineResult::Halt(Ok(debug));
        }

        let mut legalizer_ctx = LegalizerContext::new(
            ctx.ctx,
        );

        let mut legalizer = AsmLegalizer::new(&mut program);
        legalizer.legalize(&mut legalizer_ctx);

        if ctx.stage == PipelineStage::AssemblyLegalization {
            let debug = format!("{:#?}", program);
            return PipelineResult::Halt(Ok(debug));
        }

        let emitter = CodeEmitter::new();
        let program = emitter.emit(program);

        PipelineResult::Continue(program)
    }
}