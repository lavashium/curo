use super::*;
use crate::asm::*;
use common::*;

impl Factory<(), AsmFunction, AllocatorContext<'_, '_>> for AllocatorAllocations {
    fn run(function: &mut AsmFunction, ctx: &mut AllocatorContext) {
        for instr in function.instructions_mut() {
            Self::run(instr, ctx);
        }
    }
}