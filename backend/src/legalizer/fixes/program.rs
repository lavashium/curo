use super::*;
use crate::asm::*;
use crate::*;
use common::*;

impl Factory<(), AsmProgram, LegalizerContext<'_, '_>> for LegalizerLegalizations {
    fn run(program: &mut AsmProgram, ctx: &mut LegalizerContext) {
        for function in program.function_definitions_mut() {
            LegalizerLegalizations::run(function, ctx);
        }
    }
}