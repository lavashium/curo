use crate::*;
use super::*;
use common::*;
use language::*;

impl<'a> ParserRules<'a> {
    pub fn parse_declaration(&mut self, ctx: &mut ParserContext) -> Option<AstDeclaration> {
        <Self as Factory<Option<AstDeclaration>, Self, ParserContext>>::run(self, ctx)
    }
}

impl<'a> Factory<Option<AstDeclaration>, Self, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut Self, ctx: &mut ParserContext) -> Option<AstDeclaration> {
        let start_span = rules.parser.source_tokens.peek()?.get_span();

        rules.expect(ctx, TokenKind::Keyword(KeywordKind::Int))?;
        let name = rules.unwrap_identifier(ctx)?;

        if rules.parser.source_tokens.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::OpenParen) {
            rules.parser.source_tokens.consume()?;
            let params = rules.parse_param_list(ctx)?;
            rules.expect(ctx, TokenKind::Punctuation(PunctuationKind::CloseParen))?;

            let body = if rules.parser.source_tokens.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::Semicolon) {
                rules.parser.source_tokens.consume()?;
                None
            } else if rules.parser.source_tokens.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::OpenBrace) {
                Some(rules.parse_block(ctx)?)
            } else {
                return None;
            };

            let end_span = rules.parser.source_tokens.peek()?.get_span();
            let span = combine_spans!(start_span, end_span);
            Some(AstDeclaration::FunDecl(AstFunctionDeclaration::new(
                name, 
                params, 
                body,
                span
            )))
        } else {
            let init = if rules.parser.source_tokens.peek()?.kind() == &TokenKind::Operator(OperatorKind::Equal) {
                rules.parser.source_tokens.consume()?;
                Some(rules.parse_expression(ctx)?)
            } else {
                None
            };

            rules.expect(ctx, TokenKind::Punctuation(PunctuationKind::Semicolon))?;
            let end_span = rules.parser.source_tokens.peek()?.get_span();
            let span = combine_spans!(start_span, end_span);
            
            Some(AstDeclaration::VarDecl(AstVariableDeclaration::new(
                name,
                init,
                span
            )))
        }
    }
}
