use super::*;
use language::*;

pub trait BlockParser {
    fn parse_block(&mut self) -> ParseResult<AstBlock>;
}

impl<'a> BlockParser for ParserRules<'a> {
    fn parse_block(&mut self) -> ParseResult<AstBlock> {
        self.expect(TokenKind::Punctuation(PunctuationKind::OpenBrace))?;
        let mut items = Vec::new();

        while self.parser.source_tokens.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::CloseBrace) {
            items.push(self.parse_block_item()?);
        }
        self.expect(TokenKind::Punctuation(PunctuationKind::CloseBrace))?;
        Some(AstBlock::new(items))
    }
}