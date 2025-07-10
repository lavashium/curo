use super::*;
use language::*;

pub trait BlockParser {
    fn parse_block(&mut self) -> ParseResult<AstBlock>;
}

impl<'a> BlockParser for ParserRules<'a> {
    fn parse_block(&mut self) -> ParseResult<AstBlock> {
        self.expect(token_punctuation!(OpenBrace))?;

        let mut items = Vec::new();
        while !matches!(self.parser.source_tokens.peek()?.kind(), token_punctuation!(CloseBrace)) {
            items.push(self.parse_block_item()?);
        }

        self.expect(token_punctuation!(CloseBrace))?;
        Some(AstBlock::new(items))
    }
}
