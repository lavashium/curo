use super::*;

pub trait BlockItemParser {
    fn parse_block_item(&mut self) -> ParseResult<AstBlockItem>;
}

impl<'a> BlockItemParser for ParserRules<'a> {
    fn parse_block_item(&mut self) -> ParseResult<AstBlockItem> {
        match self.parser.source_tokens.peek()?.kind() {
            TokenKind::Keyword(KeywordKind::Int) => {
                let decl = self.parse_declaration()?;
                Some(AstBlockItem::Declaration(decl))
            }
            _ => {
                let stmt = self.parse_statement()?;
                Some(AstBlockItem::Statement(stmt))
            }
        }
    }
}
