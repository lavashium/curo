use super::rules::*;
use common::error::manager::DiagnosticsManager;
use language::ast::*;
use language::token::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
pub struct Parser {
    pub source_tokens: TokenStream,
}

impl Parser {
    pub fn parse(&mut self, diagnostics: &mut DiagnosticsManager) -> Option<AstProgram> {
        ParserRules::new(self, diagnostics).parse_program()
    }
}
