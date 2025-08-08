use crate::*;
use super::*;
use common::*;
use language::*;

impl<'scp, 'ctx> Factory<Option<AstFunctionDeclaration>, TokenStream> for ParserRules<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstFunctionDeclaration> {
        let start_span = stream.peek()?.get_span();
        Self::expect(stream, ctx, TokenKind::Keyword(KeywordKind::Int))?;
        let name = Self::unwrap_identifier(stream, ctx)?;
        Self::expect(stream, ctx, TokenKind::Punctuation(PunctuationKind::OpenParen))?;
        let params = try_apply!(Self, _, stream, ctx);
        Self::expect(stream, ctx, TokenKind::Punctuation(PunctuationKind::CloseParen))?;
        
        let body = if stream.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::Semicolon) {
            stream.consume()?;
            None
        } else {
            Some(try_apply!(Self, _, stream, ctx))
        };
        
        let end_span = stream.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);

        Some(AstFunctionDeclaration::new(
            name, 
            params, 
            body,
            None,
            span
        ))
    }
}