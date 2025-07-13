use super::*;
use language::*;

pub trait FunctionDeclParser {
    fn parse_function_declaration(&mut self) -> ParseResult<AstFunctionDeclaration>;
}

impl<'a> FunctionDeclParser for ParserRules<'a> {
    fn parse_function_declaration(&mut self) -> ParseResult<AstFunctionDeclaration> {
        self.expect(TokenKind::Keyword(KeywordKind::Int))?;
        let name = self.unwrap_identifier()?;
        self.expect(TokenKind::Punctuation(PunctuationKind::OpenParen))?;
        let params = self.parse_param_list()?;
        self.expect(TokenKind::Punctuation(PunctuationKind::CloseParen))?;
        
        let body = if self.parser.source_tokens.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::Semicolon) {
            self.parser.source_tokens.consume()?;
            None
        } else {
            Some(self.parse_block()?)
        };
        
        Some(AstFunctionDeclaration::new(name, params, body))
    }
}