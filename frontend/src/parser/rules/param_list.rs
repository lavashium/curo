use crate::*;
use super::*;
use common::*;
use language::*;

impl<'scp, 'ctx> Factory<Option<Vec<String>>, TokenStream> for ParserRules<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<Vec<String>> {
        let mut params = Vec::new();
        
        if stream.peek()?.kind() == &TokenKind::Keyword(KeywordKind::Void) {
            stream.consume()?;
            return Some(params);
        }
        
        loop {
            Self::expect(stream, ctx, TokenKind::Keyword(KeywordKind::Int))?;
            let id = Self::unwrap_identifier(stream, ctx)?;
            params.push(id);
            
            if stream.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::Comma) {
                break;
            }
            stream.consume()?;
        }
        
        Some(params)
    }
}