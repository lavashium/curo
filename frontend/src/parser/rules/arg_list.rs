use crate::*;
use super::*;
use common::*;
use language::*;

impl<'a> ParserRules<'a> {
    pub fn parse_argument_list(&mut self, ctx: &mut ParserContext) -> Option<Vec<Box<AstExpression>>> {
        <Self as Factory<Option<Vec<Box<AstExpression>>>, Self, ParserContext>>::run(self, ctx)
    }
}

impl<'a> Factory<Option<Vec<Box<AstExpression>>>, Self, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut Self, ctx: &mut ParserContext) -> Option<Vec<Box<AstExpression>>> {
        let mut args = Vec::new();
        if rules.parser.source_tokens.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::CloseParen) {
            args.push(Box::new(rules.parse_expression(ctx)?));
            while rules.parser.source_tokens.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::Comma) {
                rules.parser.source_tokens.consume()?;
                args.push(Box::new(rules.parse_expression(ctx)?));
            }
        }
        Some(args)
    }
}