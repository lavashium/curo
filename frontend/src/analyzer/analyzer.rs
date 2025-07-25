use accessors::accessors;
use common::*;
use constructors::constructors;
use zawarudo::zawarudo;
use language::*;
use crate::*;
use super::checks::*;

#[accessors]
#[constructors]
pub struct Analyzer<'a> {
    program: &'a mut TypedProgram,
}

impl<'a> Analyzer<'a> {
    #[zawarudo(label = "Semantic Analyzer")]
    pub fn analyze(&mut self, ctx: &mut AnalyzerContext) {
        CHECKS::run(self.program, ctx)
    }
} 