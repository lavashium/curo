use crate::compiler::*;
use common::*;
use language::*;
use frontend::*;

use super::*;

pub struct ParserStage;

impl Factory<PipelineResult<AstProgram>, TokenStream, PipelineContext<'_, '_>> for ParserStage {
    fn run(tokens: &mut TokenStream, ctx: &mut PipelineContext) -> PipelineResult<AstProgram> {
        let mut parser_ctx = ParserContext::new(
            ctx.ctx, 
            0,
        );

        let mut parser = Parser::new(tokens);
        let program = parser.parse(&mut parser_ctx);

        if !ctx.ctx.diagnostics.is_empty() {
            let _ = ctx.ctx.diagnostics.report();
            return PipelineResult::Halt(Err(ErrCode::ParserError));
        }

        let program = program.expect("Parser returned None despite no diagnostics errors");

        if ctx.stage == PipelineStage::Parser {
            let debug = format!("{:#?}", program);
            return PipelineResult::Halt(Ok(debug));
        }

        PipelineResult::Continue(program)
    }
}