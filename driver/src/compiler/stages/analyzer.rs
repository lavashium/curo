use crate::compiler::*;
use common::*;
use language::*;
use frontend::*;

use super::*;

pub struct AnalysisStage;

impl Factory<PipelineResult<TypedProgram>, AstProgram, PipelineContext<'_, '_>> for AnalysisStage {
    fn run(program: &mut AstProgram, ctx: &mut PipelineContext) -> PipelineResult<TypedProgram> {
        let mut analyzer_ctx = AnalyzerContext::new(
            ctx.ctx,
            Vec::new(),
            0,
            false
        );

        analyzer_ctx.push_scope(false);

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