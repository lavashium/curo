use crate::parser::rules::*;
use language::*;

impl<'scp, 'ctx> ParserRules<'scp, 'ctx>  {
    pub fn parse_do(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstStatement> {
        let start_span = stream.peek()?.get_span();
        Self::expect(stream, ctx, token_keyword!(Do))?;
        let body = try_apply!(Self, _, stream, ctx);
        Self::expect(stream, ctx, token_keyword!(While))?;
        Self::expect(stream, ctx, token_punctuation!(OpenParen))?;
        let condition = try_apply!(Self, _, stream, ctx);
        Self::expect(stream, ctx, token_punctuation!(CloseParen))?;
        Self::expect(stream, ctx, token_punctuation!(Semicolon))?;
        let end_span = stream.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);

        Some(AstStatement::new_do_while (
            condition,
            Box::new(body),
            String::new(),
            span
        ))
    }
}

