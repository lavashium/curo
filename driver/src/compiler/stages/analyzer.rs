use std::marker::PhantomData;
use common::*;
use language::*;
use frontend::*;

use super::*;

pub struct AnalysisStage<'scp, 'ctx> {
    _driver: PhantomData<PipelineContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for AnalysisStage<'scp, 'ctx> {
    type Context = PipelineContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<PipelineResult<TypedProgram>, AstProgram> for AnalysisStage<'scp, 'ctx> {
    fn run(program: &mut AstProgram, ctx: &mut PipelineContext<'scp, 'ctx>) -> PipelineResult<TypedProgram> {
        let mut analyzer_ctx = AnalyzerContext::new(
            ctx.ctx,
            IdentifierMap::new(),
            0,
            false,
            false,
            None
        );

        let mut program = program.to_typed();
        let mut analyzer = Analyzer::new(&mut program);
        analyzer.analyze(&mut analyzer_ctx);

        if !ctx.ctx.diagnostics.is_empty() {
            let _ = ctx.ctx.diagnostics.report();
            return PipelineResult::Halt(Err(ErrCode::SemanticError));
        }

        if ctx.stage == PipelineStage::Analysis {
            let debug = format!("{:#?}", program);
            return PipelineResult::Halt(Ok(debug));
        }

        PipelineResult::Continue(program)
    }
}