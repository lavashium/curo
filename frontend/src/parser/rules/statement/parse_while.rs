use crate::parser::rules::*;
use language::*;

impl<'scp, 'ctx> ParserRules<'scp, 'ctx>{
    pub fn parse_while(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstStatement> {
        let start_span = stream.peek()?.get_span();
        Self::expect(stream, ctx, token_keyword!(While))?;
        Self::expect(stream, ctx, token_punctuation!(OpenParen))?;
        let condition = try_apply!(Self, _, stream, ctx);
        Self::expect(stream, ctx, token_punctuation!(CloseParen))?;
        let body = try_apply!(Self, _, stream, ctx);
        let end_span = stream.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);
        Some(AstStatement::new_while(
            condition,
            Box::new(body),
            String::new(),
            span
        ))
    }
}