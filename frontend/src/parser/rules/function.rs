use super::*;
use language::*;

pub trait FunctionParser {
    fn parse_function(&mut self) -> ParseResult<AstFunction>;
}

impl<'a> FunctionParser for ParserRules<'a> {
    fn parse_function(&mut self) -> ParseResult<AstFunction> {
        self.expect(token_keyword!(Int))?;
        let identifier = self.unwrap_identifier()?;
        self.expect(token_punctuation!(OpenParen))?;
        self.expect(token_keyword!(Void))?;
        self.expect(token_punctuation!(CloseParen))?;

        let body = self.parse_block()?;

        Some(AstFunction::new(identifier, body))
    }
}
