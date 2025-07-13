use super::*;
use language::*;

pub trait ForInitParser {
    fn parse_for_init(&mut self) -> ParseResult<AstForInit>;
}

impl<'a> ForInitParser for ParserRules<'a> {
    fn parse_for_init(&mut self) -> ParseResult<AstForInit> {
        if self.parser.source_tokens.peek()?.kind() == &TokenKind::Keyword(KeywordKind::Int) {
            let var_decl = self.parse_variable_declaration()?;
            self.expect(TokenKind::Punctuation(PunctuationKind::Semicolon))?;
            Some(AstForInit::InitDeclaration(var_decl))
        } else {
            let expr = if self.parser.source_tokens.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::Semicolon) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            self.expect(TokenKind::Punctuation(PunctuationKind::Semicolon))?;
            Some(AstForInit::InitExpression(expr))
        }
    }
}