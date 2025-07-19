use crate::*;
use super::*;
use common::*;
use language::*;

impl<'a> ParserRules<'a> {
    pub fn parse_block(&mut self, ctx: &mut ParserContext) -> Option<AstBlock> {
        <Self as Factory<Option<AstBlock>, Self, ParserContext>>::run(self, ctx)
    }
}

impl<'a> Factory<Option<AstBlock>, Self, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut Self, ctx: &mut ParserContext) -> Option<AstBlock> {
        let start_span = rules.parser.source_tokens.peek()?.get_span();

        rules.expect(ctx, TokenKind::Punctuation(PunctuationKind::OpenBrace))?;
        let mut items = Vec::new();

        while rules.parser.source_tokens.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::CloseBrace) {
            items.push(rules.parse_block_item(ctx)?);
        }

        rules.expect(ctx, TokenKind::Punctuation(PunctuationKind::CloseBrace))?;
        let end_span = rules.parser.source_tokens.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);

        Some(AstBlock::new(
            items,
            span
        ))
    }
}