use super::*;
use common::*;
use language::*;

mod parse_return;
mod parse_if;
mod parse_break;
mod parse_continue;
mod parse_while;
mod parse_do;
mod parse_for;

use parse_return::*;
use parse_if::*;
use parse_break::*;
use parse_continue::*;
use parse_while::*;
use parse_do::*;
use parse_for::*;

impl<'a> ParserRules<'a> {
    pub fn parse_statement(&mut self, ctx: &mut ParserContext) -> Option<AstStatement> {
        <Self as Factory<Option<AstStatement>, Self, ParserContext>>::run(self, ctx)
    }
}

impl<'a> Factory<Option<AstStatement>, Self, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut Self, ctx: &mut ParserContext) -> Option<AstStatement> {
        match rules.parser.source_tokens.peek()?.kind() {
            TokenKind::Keyword(KeywordKind::Return)   => parse_return  (rules, ctx),
            TokenKind::Keyword(KeywordKind::If)       => parse_if      (rules, ctx),
            TokenKind::Keyword(KeywordKind::Break)    => parse_break   (rules, ctx),
            TokenKind::Keyword(KeywordKind::Continue) => parse_continue(rules, ctx),
            TokenKind::Keyword(KeywordKind::While)    => parse_while   (rules, ctx),
            TokenKind::Keyword(KeywordKind::Do)       => parse_do      (rules, ctx),
            TokenKind::Keyword(KeywordKind::For)      => parse_for     (rules, ctx),
            TokenKind::Punctuation(PunctuationKind::Semicolon) => {
                rules.expect(ctx, token_punctuation!(Semicolon))?;
                Some(AstStatement::new_null())
            }
            TokenKind::Punctuation(PunctuationKind::OpenBrace) => {
                let start_span = &rules.parser.source_tokens.peek()?.get_span();
                let block = rules.parse_block(ctx)?;
                let end_span = &rules.parser.source_tokens.peek()?.get_span();
                let span = combine_spans!(start_span, end_span);
                Some(AstStatement::new_compound(
                    block,
                    span
                ))
            }
            _ => {
                let start_span = &rules.parser.source_tokens.peek()?.get_span();
                let expr = rules.parse_expression(ctx)?;
                rules.expect(ctx, token_punctuation!(Semicolon))?;
                let end_span = &rules.parser.source_tokens.peek()?.get_span();
                let span = combine_spans!(start_span, end_span);
                Some(AstStatement::new_expression(
                    expr,
                    span
                ))
            }
        }
    }
}
