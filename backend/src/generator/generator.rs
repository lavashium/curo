use super::casts::*;
use crate::asm::*;
use constructors::constructors;
use common::*;
use language::*;
use zawarudo::*;

#[constructors]
pub struct AsmGenerator<'scp> {
    source_program: &'scp mut TacProgram
}

impl<'scp> AsmGenerator<'scp> {
    #[zawarudo(label = "AsmGenerator")]
    pub fn generate(&mut self, ctx: &mut ()) -> AsmProgram {
        GeneratorCasts::run(self.source_program, ctx)
    }
}
