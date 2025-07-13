use super::*;
use language::*;

pub trait VariableDeclParser {
    fn parse_variable_declaration(&mut self) -> ParseResult<AstVariableDeclaration>;
}

impl<'a> VariableDeclParser for ParserRules<'a> {
    fn parse_variable_declaration(&mut self) -> ParseResult<AstVariableDeclaration> {
        self.expect(TokenKind::Keyword(KeywordKind::Int))?;
        let name = self.unwrap_identifier()?;
        let init = if self.parser.source_tokens.peek()?.kind() == &TokenKind::Operator(OperatorKind::Equal) {
            self.parser.source_tokens.consume()?;
            Some(self.parse_expression()?)
        } else {
            None
        };
        Some(AstVariableDeclaration::new(name, init))
    }
}