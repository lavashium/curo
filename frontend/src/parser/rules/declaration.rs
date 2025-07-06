use super::*;

pub trait DeclarationParser {
    fn parse_declaration(&mut self) -> ParseResult<AstDeclaration>;
}

impl<'a> DeclarationParser for ParserRules<'a> {
    fn parse_declaration(&mut self) -> ParseResult<AstDeclaration> {
        self.expect(token_keyword!(Int))?;
        let name = self.unwrap_identifier()?;

        let init = if self.parser.source_tokens().peek()?.kind() == &token_operator!(Equal) {
            self.expect(token_operator!(Equal))?;
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(token_punctuation!(Semicolon))?;

        Some(AstDeclaration::new(
            name, 
            init
        ))
    }
}
