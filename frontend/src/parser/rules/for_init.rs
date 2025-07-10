use super::*;
use language::*;

pub trait ForInitParser {
    fn parse_for_init(&mut self) -> ParseResult<AstForInit>;
}

impl<'a> ForInitParser for ParserRules<'a> {
    fn parse_for_init(&mut self) -> ParseResult<AstForInit> {
        let next = self.parser.source_tokens.peek()?.get_kind();

        if let TokenKind::Keyword(KeywordKind::Int) = next {
            let decl = self.parse_declaration()?;
            return Some(AstForInit::InitDeclaration(decl));
        }

        if let TokenKind::Punctuation(PunctuationKind::Semicolon) = next {
            self.expect(token_punctuation!(Semicolon))?;
            return Some(AstForInit::InitExpression(None));
        }

        let expr = self.parse_expression()?;
        self.expect(token_punctuation!(Semicolon))?;
        Some(AstForInit::InitExpression(Some(expr)))
    }
}