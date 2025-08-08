use crate::parser::rules::*;
use language::*;

impl<'scp, 'ctx> ParserRules<'scp, 'ctx>  {
    pub fn parse_for(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstStatement> {
        let start_span = stream.peek()?.get_span();
        Self::expect(stream, ctx, token_keyword!(For))?;
        Self::expect(stream, ctx, token_punctuation!(OpenParen))?;
        let for_init = try_apply!(Self, _, stream, ctx);

        let condition = match stream.peek()?.kind() {
            TokenKind::Punctuation(PunctuationKind::Semicolon) => {
                Self::expect(stream, ctx, token_punctuation!(Semicolon))?;
                None
            }
            _ => {
                let cond = try_apply!(Self, _, stream, ctx);
                Self::expect(stream, ctx, token_punctuation!(Semicolon))?;
                Some(cond)
            }
        };

        let post = match stream.peek()?.kind() {
            TokenKind::Punctuation(PunctuationKind::CloseParen) => {
                Self::expect(stream, ctx, token_punctuation!(CloseParen))?;
                None
            }
            _ => {
                let post_expr = try_apply!(Self, _, stream, ctx);
                Self::expect(stream, ctx, token_punctuation!(CloseParen))?;
                Some(post_expr)
            }
        };

        let body = try_apply!(Self, _, stream, ctx);
        let end_span = stream.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);

        Some(AstStatement::new_for(
            for_init,
            condition,
            post,
            Box::new(body),
            String::new(),
            span
        ))
    }
}