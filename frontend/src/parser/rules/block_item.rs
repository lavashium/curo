use crate::*;
use super::*;
use common::*;
use language::*;

impl<'a> ParserRules<'a> {
    pub fn parse_block_item(&mut self, ctx: &mut ParserContext) -> Option<AstBlockItem> {
        <Self as Factory<Option<AstBlockItem>, Self, ParserContext>>::run(self, ctx)
    }
}

impl<'a> Factory<Option<AstBlockItem>, Self, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut Self, ctx: &mut ParserContext) -> Option<AstBlockItem> {
        if rules.parser.source_tokens.peek()?.kind() == &TokenKind::Keyword(KeywordKind::Int) {
            Some(AstBlockItem::Declaration(rules.parse_declaration(ctx)?))
        } else {
            Some(AstBlockItem::Statement(rules.parse_statement(ctx)?))
        }
    }
}