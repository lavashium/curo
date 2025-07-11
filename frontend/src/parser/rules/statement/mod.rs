use super::*;

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

pub trait StatementParser {
    fn parse_statement(&mut self) -> ParseResult<AstStatement>;
}

impl<'a> StatementParser for ParserRules<'a> {
    fn parse_statement(&mut self) -> ParseResult<AstStatement> {
        match self.parser.source_tokens.peek()?.kind() {
            TokenKind::Keyword(KeywordKind::Return)   => parse_return(self),
            TokenKind::Keyword(KeywordKind::If)       => parse_if(self),
            TokenKind::Keyword(KeywordKind::Break)    => parse_break(self),
            TokenKind::Keyword(KeywordKind::Continue) => parse_continue(self),
            TokenKind::Keyword(KeywordKind::While)    => parse_while(self),
            TokenKind::Keyword(KeywordKind::Do)       => parse_do(self),
            TokenKind::Keyword(KeywordKind::For)      => parse_for(self),
            TokenKind::Punctuation(PunctuationKind::Semicolon) => {
                self.expect(token_punctuation!(Semicolon))?;
                Some(AstStatement::new_null())
            }
            TokenKind::Punctuation(PunctuationKind::OpenBrace) => {
                let start_span = &self.parser.source_tokens.peek()?.get_span();
                let block = self.parse_block()?;
                let end_span = &self.parser.source_tokens.peek()?.get_span();
                let span = combine_spans!(start_span, end_span);
                Some(AstStatement::new_compound(
                    block,
                    span
                ))
            }
            _ => {
                let start_span = &self.parser.source_tokens.peek()?.get_span();
                let expr = self.parse_expression()?;
                self.expect(token_punctuation!(Semicolon))?;
                let end_span = &self.parser.source_tokens.peek()?.get_span();
                let span = combine_spans!(start_span, end_span);
                Some(AstStatement::new_expression(
                    expr,
                    span
                ))
            }
        }
    }
}
