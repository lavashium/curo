use crate::*;
use super::*;
use common::*;
use language::*;

impl<'scp, 'ctx> Factory<Option<AstProgram>, TokenStream> for ParserRules<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstProgram> {
        let mut declarations = Vec::new();
        while stream.peek()?.kind() != &TokenKind::EOF {
            declarations.push(try_apply!(Self, _, stream, ctx));
        }
        Self::expect(stream, ctx, TokenKind::EOF)?;
        Some(AstProgram::new(declarations))
    }
}