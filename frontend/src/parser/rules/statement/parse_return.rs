use crate::parser::rules::*;
use language::*;

impl<'scp, 'ctx> ParserRules<'scp, 'ctx>  {
    pub fn parse_return(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstStatement> {
        let start_span = stream.peek()?.get_span();
        Self::expect(stream, ctx, token_keyword!(Return))?;
        let expression = try_apply!(Self, _, stream, ctx);
        Self::expect(stream, ctx, token_punctuation!(Semicolon))?;
        let end_span = stream.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);
        Some(AstStatement::new_return(
            expression,
            span
        ))
    }
}