use std::collections::HashMap;

use crate::compiler::*;
use common::*;
use language::*;
use backend::*;

use super::*;

pub struct X86_64;

impl Factory<PipelineResult<String>, TacProgram, PipelineContext<'_, '_>> for X86_64 {
    fn run(program: &mut TacProgram, ctx: &mut PipelineContext) -> PipelineResult<String> {
        
        let mut generator = AsmGenerator::new(program);
        let mut program = generator.generate(&mut ());

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