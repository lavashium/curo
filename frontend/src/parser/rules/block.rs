use crate::*;
use super::*;
use common::*;
use language::*;

impl<'scp, 'ctx> Factory<Option<AstBlock>, TokenStream> for ParserRules<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstBlock> {
        let start_span = stream.peek()?.get_span();

        Self::expect(stream, ctx, TokenKind::Punctuation(PunctuationKind::OpenBrace))?;
        let mut items = Vec::new();

        while stream.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::CloseBrace) {
            items.push(try_apply!(Self, _, stream, ctx));
        }

        Self::expect(stream, ctx, TokenKind::Punctuation(PunctuationKind::CloseBrace))?;
        let end_span = stream.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);

        Some(AstBlock::new(
            items,
            span
        ))
    }
}