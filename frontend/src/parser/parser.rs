use crate::ParserContext;
use super::rules::*;
use accessors::accessors;
use constructors::constructors;
use language::*;
use common::*;
use zawarudo::zawarudo;

#[accessors]
#[constructors]
pub struct Parser<'scp> {
    pub source_tokens: &'scp mut TokenStream,
}

impl<'scp> Parser<'scp> {
    #[zawarudo(label = "Parser")]
    pub fn parse(&mut self, ctx: &mut ParserContext) -> Option<AstProgram> {
        ParserRules::run(self.source_tokens, ctx)
    }
}
