use crate::*;
use super::*;
use common::*;
use language::*;

impl<'a> ParserRules<'a> {
    pub fn parse_function_declaration(&mut self, ctx: &mut ParserContext) -> Option<AstFunctionDeclaration> {
        <Self as Factory<Option<AstFunctionDeclaration>, Self, ParserContext>>::run(self, ctx)
    }
}

impl<'a> Factory<Option<AstFunctionDeclaration>, Self, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut Self, ctx: &mut ParserContext) -> Option<AstFunctionDeclaration> {
        let start_span = rules.parser.source_tokens.peek()?.get_span();
        rules.expect(ctx, TokenKind::Keyword(KeywordKind::Int))?;
        let name = rules.unwrap_identifier(ctx)?;
        rules.expect(ctx, TokenKind::Punctuation(PunctuationKind::OpenParen))?;
        let params = rules.parse_param_list(ctx)?;
        rules.expect(ctx, TokenKind::Punctuation(PunctuationKind::CloseParen))?;
        
        let body = if rules.parser.source_tokens.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::Semicolon) {
            rules.parser.source_tokens.consume()?;
            None
        } else {
            Some(rules.parse_block(ctx)?)
        };
        
        let end_span = rules.parser.source_tokens.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);

        Some(AstFunctionDeclaration::new(
            name, 
            params, 
            body,
            span
        ))
    }
}