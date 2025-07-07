use super::rules::*;
use accessors::accessors;
use common::error::manager::DiagnosticsManager;
use constructors::constructors;
use language::ast::*;
use language::token::*;
use zawarudo::zawarudo;

#[accessors]
#[constructors]
pub struct Parser {
    pub source_tokens: TokenStream,
}

impl Parser {
    #[zawarudo(label = "Parser")]
    pub fn parse(&mut self, diagnostics: &mut DiagnosticsManager) -> Option<AstProgram> {
        ParserRules::new(self, diagnostics).parse_program()
    }
}
