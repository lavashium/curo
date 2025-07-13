use super::*;
use language::*;

pub trait ArgumentListParser {
    fn parse_argument_list(&mut self) -> ParseResult<Vec<Box<AstExpression>>>;
}

impl<'a> ArgumentListParser for ParserRules<'a> {
    fn parse_argument_list(&mut self) -> ParseResult<Vec<Box<AstExpression>>> {
        let mut args = Vec::new();
        if self.parser.source_tokens.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::CloseParen) {
            args.push(Box::new(self.parse_expression()?));
            while self.parser.source_tokens.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::Comma) {
                self.parser.source_tokens.consume()?;
                args.push(Box::new(self.parse_expression()?));
            }
        }
        Some(args)
    }
}