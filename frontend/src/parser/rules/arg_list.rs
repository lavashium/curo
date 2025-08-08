use crate::*;
use super::*;
use common::*;
use language::*;

impl<'scp, 'ctx> Factory<Option<Vec<Box<AstExpression>>>, TokenStream> for ParserRules<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<Vec<Box<AstExpression>>> {
        let mut args = Vec::new();
        if stream.peek()?.kind() != &TokenKind::Punctuation(PunctuationKind::CloseParen) {
            let arg = ParserRules::parse_expression(stream, ctx)?;
            args.push(Box::new(arg));
            while stream.peek()?.kind() == &TokenKind::Punctuation(PunctuationKind::Comma) {
                stream.consume()?;
                let arg = ParserRules::parse_expression(stream, ctx)?;
                args.push(Box::new(arg));
            }
        }
        Some(args)
    }
}