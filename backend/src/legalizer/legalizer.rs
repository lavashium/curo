use super::*;
use super::fixes::*;
use crate::asm::*;
use accessors::accessors;
use common::Factory;
use constructors::constructors;
use zawarudo::zawarudo;

#[accessors]
#[constructors]
pub struct AsmLegalizer<'scp> {
    source_asm: &'scp mut AsmProgram,
}

impl<'scp> AsmLegalizer<'scp> {
    #[zawarudo(label = "Assembly Legalizer")]
    pub fn legalize(&mut self, ctx: &mut LegalizerContext) {
        LegalizerLegalizations::run(self.source_asm, ctx)
    }
}

