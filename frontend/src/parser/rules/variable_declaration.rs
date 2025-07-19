use crate::*;
use super::*;
use common::*;
use language::*;

impl<'a> ParserRules<'a> {
    pub fn parse_variable_declaration(&mut self, ctx: &mut ParserContext) -> Option<AstVariableDeclaration> {
        <Self as Factory<Option<AstVariableDeclaration>, Self, ParserContext>>::run(self, ctx)
    }
}

impl<'a> Factory<Option<AstVariableDeclaration>, Self, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut Self, ctx: &mut ParserContext) -> Option<AstVariableDeclaration> {
        let start_span = rules.parser.source_tokens.peek()?.get_span();
        rules.expect(ctx, TokenKind::Keyword(KeywordKind::Int))?;
        let name = rules.unwrap_identifier(ctx)?;
        let init = if rules.parser.source_tokens.peek()?.kind() == &TokenKind::Operator(OperatorKind::Equal) {
            rules.parser.source_tokens.consume()?;
            Some(rules.parse_expression(ctx)?)
        } else {
            None
        };

        let end_span = rules.parser.source_tokens.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);
        
        Some(AstVariableDeclaration::new(
            name, 
            init,
            span
        ))
    }
}