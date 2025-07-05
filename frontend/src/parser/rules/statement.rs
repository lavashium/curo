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

            _ => {
                let token = self.parser.source_tokens.peek()?;
                self.diagnostics.push(
                    Diagnostic::error(
                        token.get_span(),
                        DiagnosticKind::new_unknown_token(token.clone()),
                    )
                    .with(Diagnostic::note(
                        token.get_span(),
                        "expected a statement here",
                    )),
                );
                None
            }
        }
    }
}
