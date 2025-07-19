use crate::*;
use super::*;
use common::*;
use language::*;

impl<'a> ParserRules<'a> {
    pub fn parse_program(&mut self, ctx: &mut ParserContext) -> Option<AstProgram> {
        <Self as Factory<Option<AstProgram>, Self, ParserContext>>::run(self, ctx)
    }
}

impl<'a> Factory<Option<AstProgram>, Self, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut Self, ctx: &mut ParserContext) -> Option<AstProgram> {
        let mut functions = Vec::new();
        while rules.parser.source_tokens.peek()?.kind() != &TokenKind::EOF {
            functions.push(rules.parse_function_declaration(ctx)?);
        }
        rules.expect(ctx, TokenKind::EOF)?;
        Some(AstProgram::new(functions))
    }
}