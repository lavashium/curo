use crate::*;
use super::*;
use common::*;
use language::*;

impl<'scp, 'ctx> Factory<Option<AstVariableDeclaration>, TokenStream> for ParserRules<'scp, 'ctx>{
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstVariableDeclaration> {
        let start_span = stream.peek()?.get_span();
        Self::expect(stream, ctx, TokenKind::Keyword(KeywordKind::Int))?;
        let name = Self::unwrap_identifier(stream, ctx)?;
        let init = if stream.peek()?.kind() == &TokenKind::Operator(OperatorKind::Equal) {
            stream.consume()?;
            Some(try_apply!(Self, _, stream, ctx))
        } else {
            None
        };

        let end_span = stream.peek()?.get_span();
        let span = combine_spans!(start_span, end_span);
        
        Some(AstVariableDeclaration::new(
            name, 
            init,
            None,
            span
        ))
    }
}