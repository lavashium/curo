use crate::ParserContext;
use super::rules::*;
use accessors::accessors;
use constructors::constructors;
use language::ast::*;
use language::token::*;
use zawarudo::zawarudo;

#[accessors]
#[constructors]
pub struct Parser<'scp> {
    pub source_tokens: &'scp mut TokenStream,
}

impl<'scp> Parser<'scp> {
    #[zawarudo(label = "Parser")]
    pub fn parse(&'scp mut self, ctx: &mut ParserContext) -> Option<AstProgram> {
        ParserRules::new(self).parse_program(ctx)
    }
}
