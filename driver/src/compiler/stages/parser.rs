use std::marker::PhantomData;
use common::*;
use language::*;
use frontend::*;

use super::*;

pub struct ParserStage<'scp, 'ctx> {
    _driver: PhantomData<PipelineContext<'scp, 'ctx>>
}

impl<'scp, 'ctx> Driver for ParserStage<'scp, 'ctx> {
    type Context = PipelineContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<PipelineResult<AstProgram>, TokenStream> for ParserStage<'scp, 'ctx> {
    fn run(tokens: &mut TokenStream, ctx: &mut PipelineContext<'scp, 'ctx>) -> PipelineResult<AstProgram> {
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