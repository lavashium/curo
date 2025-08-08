use crate::*;
use super::*;
use common::*;
use language::*;

impl<'scp, 'ctx> Factory<Option<AstBlockItem>, TokenStream> for ParserRules<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstBlockItem> {
        if matches!(
            stream.peek()?.kind(),
            TokenKind::Keyword(KeywordKind::Int)
                | TokenKind::Keyword(KeywordKind::Static)
                | TokenKind::Keyword(KeywordKind::Extern)
        ) {
            Some(AstBlockItem::Declaration(try_apply!(Self, _, stream, ctx)))
        } else {
            Some(AstBlockItem::Statement(try_apply!(Self, _, stream, ctx)))
        }
    }
}