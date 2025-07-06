use super::*;
use language::*;

pub trait StatementParser {
    fn parse_statement(&mut self) -> ParseResult<AstStatement>;
}

impl<'a> StatementParser for ParserRules<'a> {
    fn parse_statement(&mut self) -> ParseResult<AstStatement> {
        match self.parser.source_tokens.peek()?.kind() {
            TokenKind::Keyword(KeywordKind::Return) => {
                self.expect(token_keyword!(Return))?;
                let expression = self.parse_expression()?;
                self.expect(token_punctuation!(Semicolon))?;
                Some(AstStatement::Return { expression })
            }

            TokenKind::Punctuation(PunctuationKind::Semicolon) => {
                self.expect(token_punctuation!(Semicolon))?;
                Some(AstStatement::Null)
            }

            _ => {
                let expr = self.parse_expression()?;
                self.expect(token_punctuation!(Semicolon))?;
                Some(AstStatement::Expression { expression: expr })
            }
        }
    }
}
