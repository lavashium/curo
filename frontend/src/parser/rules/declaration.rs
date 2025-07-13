use super::*;
use language::*;

pub trait DeclarationParser {
    fn parse_declaration(&mut self) -> ParseResult<AstDeclaration>;
}

impl<'a> DeclarationParser for ParserRules<'a> {
    fn parse_declaration(&mut self) -> ParseResult<AstDeclaration> {
        self.expect(TokenKind::Keyword(KeywordKind::Int))?;
        let name = self.unwrap_identifier()?;

        if self.parser.source_tokens.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::OpenParen) {
            self.parser.source_tokens.consume()?;
            let params = self.parse_param_list()?;
            self.expect(TokenKind::Punctuation(PunctuationKind::CloseParen))?;

            let body = if self.parser.source_tokens.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::Semicolon) {
                self.parser.source_tokens.consume()?;
                None
            } else if self.parser.source_tokens.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::OpenBrace) {
                Some(self.parse_block()?)
            } else {
                return None;
            };

            Some(AstDeclaration::FunDecl(AstFunctionDeclaration::new(name, params, body)))
        } else {
            let init = if self.parser.source_tokens.peek()?.kind() == &TokenKind::Operator(OperatorKind::Equal) {
                self.parser.source_tokens.consume()?;
                Some(self.parse_expression()?)
            } else {
                None
            };

            self.expect(TokenKind::Punctuation(PunctuationKind::Semicolon))?;
            Some(AstDeclaration::VarDecl(AstVariableDeclaration::new(name, init)))
        }
    }
}
