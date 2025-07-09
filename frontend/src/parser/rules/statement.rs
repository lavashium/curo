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
            
            TokenKind::Keyword(KeywordKind::If) => {
                self.expect(token_keyword!(If))?;
                self.expect(token_punctuation!(OpenParen))?;
                let condition = self.parse_expression()?;
                self.expect(token_punctuation!(CloseParen))?;
                let then_branch = self.parse_statement()?;
                let else_branch = match self.parser.source_tokens.peek()?.kind() {
                    TokenKind::Keyword(KeywordKind::Else) => {
                        self.expect(token_keyword!(Else))?;
                        Some(Box::new(self.parse_statement()?))
                    },
                    _ => None,
                };

                Some(AstStatement::If { 
                    condition, 
                    then_branch: Box::new(then_branch),
                    else_branch: else_branch,
                })

            }

            _ => {
                let expr = self.parse_expression()?;
                self.expect(token_punctuation!(Semicolon))?;
                Some(AstStatement::Expression { expression: expr })
            }
        }
    }
}
