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
        self.expect(token_punctuation!(OpenBrace))?;

        let mut body = Vec::new();
        loop {
            let next = self.parser.source_tokens().peek()?;
            if matches!(next.kind(), token_punctuation!(CloseBrace)) {
                break;
            }
            let item = self.parse_block_item()?;
            body.push(item);
        }

        self.expect(token_punctuation!(CloseBrace))?;

        Some(AstFunction::new(identifier, body))
    }
}
