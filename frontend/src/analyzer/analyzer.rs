use super::checks::*;
use language::*;
use common::*;
use constructors::constructors;
use accessors::accessors;
use zawarudo::zawarudo;

#[accessors]
#[constructors]
pub struct SemanticContext<'a> {
    pub diagnostics: &'a mut DiagnosticsManager,
    pub temp_gen: &'a mut TempGen,
}

#[accessors]
#[constructors]
pub struct Analyzer<'a> {
    program: &'a mut AstProgram,
    ctx: SemanticContext<'a>,
}

impl<'a> Analyzer<'a> {
    #[zawarudo(label = "Semantic Analyzer")]
    pub fn analyze(&mut self) {
        CHECKS::run_all(self.program, &mut self.ctx);
    }
} 