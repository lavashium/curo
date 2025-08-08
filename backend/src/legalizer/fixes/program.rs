use super::*;
use crate::asm::*;
use crate::*;
use common::*;

impl<'scp, 'ctx> Factory<(), AsmProgram> for LegalizerLegalizations<'scp, 'ctx> {
    fn run(program: &mut AsmProgram, ctx: &mut LegalizerContext<'scp, 'ctx>) {
        for function in program.top_level_mut() {
            LegalizerLegalizations::run(function, ctx);
        }
    }
}