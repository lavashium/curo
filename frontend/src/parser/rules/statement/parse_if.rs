use crate::parser::rules::*;
use language::*;

impl<'scp, 'ctx> ParserRules<'scp, 'ctx>  {
    pub fn parse_if(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstStatement> {
        let start_span = stream.peek()?.get_span();
        Self::expect(stream, ctx, token_keyword!(If))?;
        Self::expect(stream, ctx, token_punctuation!(OpenParen))?;
        let condition = try_apply!(Self, _, stream, ctx);
        Self::expect(stream, ctx, token_punctuation!(CloseParen))?;
        let then_branch = try_apply!(Self, _, stream, ctx);
        let else_branch = match stream.peek()?.kind() {
            TokenKind::Keyword(KeywordKind::Else) => {
                Self::expect(stream, ctx, token_keyword!(Else))?;
                Some(Box::new(try_apply!(Self, _, stream, ctx)))
            },
            _ => None,
        };
        
        let end_span = stream.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);
        Some(AstStatement::new_if(
            condition,
            Box::new(then_branch),
            else_branch,
            span
        ))
    }
}