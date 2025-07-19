use crate::compiler::*;
use common::*;
use language::*;
use frontend::*;

use super::*;

pub struct LexerStage;

impl Factory<PipelineResult<TokenStream>, &str, PipelineContext<'_, '_>> for LexerStage {
    fn run(source: &mut &str, ctx: &mut PipelineContext) -> PipelineResult<TokenStream> {
        let mut lexer_ctx = LexerContext::new(
            ctx.ctx
        );

        let mut lexer = Lexer::new(&source);
        let tokens = lexer.parse(&mut lexer_ctx);

        if !ctx.ctx.diagnostics.is_empty() {
            let _ = ctx.ctx.diagnostics.report();
            return PipelineResult::Halt(Err(ErrCode::LexerError));
        }

        if ctx.stage == PipelineStage::Lexer {
            let debug = format!("{:#?}", tokens);
            return PipelineResult::Halt(Ok(debug));
        }

        PipelineResult::Continue(tokens)
    }
}