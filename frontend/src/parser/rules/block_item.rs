use super::*;

pub trait BlockItemParser {
    fn parse_block_item(&mut self) -> ParseResult<AstBlockItem>;
}

impl<'a> BlockItemParser for ParserRules<'a> {
    fn parse_block_item(&mut self) -> ParseResult<AstBlockItem> {
        if self.parser.source_tokens.peek()?.kind() == &TokenKind::Keyword(KeywordKind::Int) {
            Some(AstBlockItem::Declaration(self.parse_declaration()?))
        } else {
            Some(AstBlockItem::Statement(self.parse_statement()?))
        }
    }
}