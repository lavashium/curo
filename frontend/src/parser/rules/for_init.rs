use crate::*;
use super::*;
use common::*;
use language::*;

impl<'a> ParserRules<'a> {
    pub fn parse_for_init(&mut self, ctx: &mut ParserContext) -> Option<AstForInit> {
        <Self as Factory<Option<AstForInit>, Self, ParserContext>>::run(self, ctx)
    }
}

impl<'a> Factory<Option<AstForInit>, Self, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut Self, ctx: &mut ParserContext) -> Option<AstForInit> {
        let start_span = rules.parser.source_tokens.peek()?.get_span();
        if rules.parser.source_tokens.peek()?.kind() == &TokenKind::Keyword(KeywordKind::Int) {
            let var_decl = rules.parse_variable_declaration(ctx)?;
            rules.expect(ctx, TokenKind::Punctuation(PunctuationKind::Semicolon))?;
            let end_span = rules.parser.source_tokens.peek()?.get_span();
            let span = combine_spans!(start_span, end_span);
            Some(AstForInit::new_init_declaration(
                var_decl,
                span
            ))
        } else {
            let expr = if rules.parser.source_tokens.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::Semicolon) {
                Some(rules.parse_expression(ctx)?)
            } else {
                None
            };
            rules.expect(ctx, TokenKind::Punctuation(PunctuationKind::Semicolon))?;
            let end_span = rules.parser.source_tokens.peek()?.get_span();
            let span = combine_spans!(start_span, end_span);
            Some(AstForInit::new_init_expression(
                expr,
                span
            ))
        }
    }
}
