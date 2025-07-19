use crate::*;
use super::*;
use common::*;
use language::*;

impl<'a> ParserRules<'a> {
    pub fn parse_param_list(&mut self, ctx: &mut ParserContext) -> Option<Vec<String>> {
        <Self as Factory<Option<Vec<String>>, Self, ParserContext>>::run(self, ctx)
    }
}

impl<'a> Factory<Option<Vec<String>>, Self, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut Self, ctx: &mut ParserContext) -> Option<Vec<String>> {
        let mut params = Vec::new();
        
        if rules.parser.source_tokens.peek()?.kind() == &TokenKind::Keyword(KeywordKind::Void) {
            rules.parser.source_tokens.consume()?;
            return Some(params);
        }
        
        loop {
            rules.expect(ctx, TokenKind::Keyword(KeywordKind::Int))?;
            let id = rules.unwrap_identifier(ctx)?;
            params.push(id);
            
            if rules.parser.source_tokens.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::Comma) {
                break;
            }
            rules.parser.source_tokens.consume()?;
        }
        
        Some(params)
    }
}